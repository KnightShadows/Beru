#![warn(missing_docs)]
//! Beru dependency resolver bridging recipes to the pubgrub solver.

use anyhow::Result;
use beru_core::cache::BeruCache;
use beru_manifest::Dependency;
use beru_recipe::resolve_recipe;
use pubgrub::range::Range;
use pubgrub::solver::{Dependencies, DependencyProvider};
use pubgrub::version::SemanticVersion;
use std::borrow::Borrow;
use std::cell::RefCell;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use tracing::{debug, info};

/// A PubGrub dependency provider for Beru packages.
pub struct BeruProvider<'a> {
    /// Reference to the global Beru cache.
    pub cache: &'a BeruCache,
    /// The project directory for resolving local path dependencies.
    pub project_dir: &'a Path,
    /// The directory of the running Beru executable for bundled recipes.
    pub beru_exe_dir: Option<PathBuf>,

    /// Maps a package name to the known `Dependency` source declaration.
    /// This is populated dynamically as we read manifests.
    pub sources: RefCell<HashMap<String, Dependency>>,

    /// Caches the dependencies for a specific package version to avoid re-fetching.
    pub deps_cache:
        RefCell<HashMap<(String, SemanticVersion), Dependencies<String, SemanticVersion>>>,

    /// Available versions for a given package name.
    pub available_versions: RefCell<HashMap<String, Vec<SemanticVersion>>>,
}

impl<'a> BeruProvider<'a> {
    /// Construct a new `BeruProvider`.
    pub fn new(cache: &'a BeruCache, project_dir: &'a Path, beru_exe_dir: Option<PathBuf>) -> Self {
        Self {
            cache,
            project_dir,
            beru_exe_dir,
            sources: RefCell::new(HashMap::new()),
            deps_cache: RefCell::new(HashMap::new()),
            available_versions: RefCell::new(HashMap::new()),
        }
    }

    /// Add a source declaration (e.g., from a parsed Beru.toml or recipe).
    pub fn add_source(&self, name: &str, dep: &Dependency) {
        let mut sources = self.sources.borrow_mut();
        sources.insert(name.to_string(), dep.clone());
    }

    /// Ensure we know about the available versions of a package.
    fn ensure_versions(&self, package: &str) -> anyhow::Result<()> {
        if self.available_versions.borrow().contains_key(package) {
            return Ok(());
        }

        debug!("Resolving available versions for {}", package);
        let versions = self.fetch_available_versions(package)?;

        self.available_versions
            .borrow_mut()
            .insert(package.to_string(), versions);
        Ok(())
    }

    fn fetch_available_versions(&self, package: &str) -> anyhow::Result<Vec<SemanticVersion>> {
        if package == "root" {
            return Ok(vec![SemanticVersion::new(0, 0, 0)]);
        }

        let sources = self.sources.borrow();
        if let Some(dep) = sources.get(package) {
            match dep {
                Dependency::Git(g) => {
                    if let Some(tag) = &g.tag {
                        if let Ok(v) = parse_version(tag) {
                            return Ok(vec![v]);
                        }
                    }
                    return Ok(vec![SemanticVersion::new(0, 0, 0)]);
                }
                Dependency::Path(_) => {
                    return Ok(vec![SemanticVersion::new(0, 0, 0)]);
                }
                Dependency::Registry(_) | Dependency::Version(_) => {}
            }
        }

        let mut versions = Vec::new();
        let index_pkg_dir = self.cache.index_dir().join(package);
        if index_pkg_dir.exists() {
            if let Ok(entries) = std::fs::read_dir(&index_pkg_dir) {
                for entry in entries.flatten() {
                    if entry.file_type().map(|ft| ft.is_dir()).unwrap_or(false) {
                        let version_str = entry.file_name().to_string_lossy().into_owned();
                        if let Ok(v) = parse_version(&version_str) {
                            let recipe_path = entry.path().join("recipe.toml");
                            if recipe_path.exists() {
                                versions.push(v);
                            }
                        }
                    }
                }
            }
        }

        if !versions.is_empty() {
            return Ok(versions);
        }

        let recipe = resolve_recipe(
            package,
            None,
            self.project_dir,
            self.beru_exe_dir.as_deref(),
            Some(&self.cache.recipes_dir()),
            Some(&self.cache.index_dir()),
        )?;

        if let Some((r, _)) = recipe {
            if let Ok(v) = parse_version(&r.package.version) {
                return Ok(vec![v]);
            }
        }

        anyhow::bail!(
            "Could not find package '{}' in index, sources, or bundled recipes",
            package
        );
    }
}

impl<'a> DependencyProvider<String, SemanticVersion> for BeruProvider<'a> {
    fn choose_package_version<T: Borrow<String>, U: Borrow<Range<SemanticVersion>>>(
        &self,
        potential_packages: impl Iterator<Item = (T, U)>,
    ) -> Result<(T, Option<SemanticVersion>), Box<dyn std::error::Error + 'static>> {
        let mut best: Option<(T, Option<SemanticVersion>)> = None;

        for (package, range) in potential_packages {
            let pkg_name = package.borrow();
            self.ensure_versions(pkg_name)
                .map_err(|e| Box::<dyn std::error::Error>::from(e.to_string()))?;

            let versions = self.available_versions.borrow();
            let mut valid_versions: Vec<SemanticVersion> = versions
                .get(pkg_name)
                .unwrap()
                .iter()
                .filter(|v| range.borrow().contains(v))
                .cloned()
                .collect();

            valid_versions.sort();
            valid_versions.reverse();

            if let Some(highest) = valid_versions.first() {
                return Ok((package, Some(*highest)));
            } else {
                best = Some((package, None));
            }
        }

        if let Some(b) = best {
            Ok(b)
        } else {
            Err("No packages to choose from".into())
        }
    }

    fn get_dependencies(
        &self,
        package: &String,
        version: &SemanticVersion,
    ) -> Result<Dependencies<String, SemanticVersion>, Box<dyn std::error::Error + 'static>> {
        let key = (package.clone(), *version);
        if let Some(deps) = self.deps_cache.borrow().get(&key) {
            return Ok(deps.clone());
        }

        info!("Fetching dependencies for {} v{}", package, version);

        let mut deps_map = pubgrub::type_aliases::Map::default();

        let version_str = version.to_string();
        let index_recipe_path = self
            .cache
            .index_dir()
            .join(package)
            .join(&version_str)
            .join("recipe.toml");

        let mut recipe = None;
        if index_recipe_path.exists() {
            if let Ok(r) = beru_recipe::Recipe::from_file(&index_recipe_path) {
                recipe = Some(r);
            }
        }

        if recipe.is_none() {
            if let Ok(Some((r, _))) = resolve_recipe(
                package,
                Some(&version_str),
                self.project_dir,
                self.beru_exe_dir.as_deref(),
                Some(&self.cache.recipes_dir()),
                Some(&self.cache.index_dir()),
            ) {
                recipe = Some(r);
            }
        }

        if let Some(r) = recipe {
            for dep_name in r.dependencies.keys() {
                let range = Range::any();
                deps_map.insert(dep_name.clone(), range);
            }
        }

        let deps = Dependencies::Known(deps_map);
        self.deps_cache.borrow_mut().insert(key, deps.clone());
        Ok(deps)
    }
}

fn parse_version(s: &str) -> anyhow::Result<SemanticVersion> {
    let clean = s.trim_start_matches('v');
    let parts: Vec<&str> = clean.split('.').collect();
    let major = parts.first().unwrap_or(&"0").parse().unwrap_or(0);
    let minor = parts.get(1).unwrap_or(&"0").parse().unwrap_or(0);
    let patch = parts.get(2).unwrap_or(&"0").parse().unwrap_or(0);
    Ok(SemanticVersion::new(major, minor, patch))
}

mod resolve;
pub use resolve::*;
