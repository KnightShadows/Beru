use anyhow::{Context, Result, bail};
use clap::Args;
use console::style;
use std::path::{Path, PathBuf};
use tracing::{debug, info};

use beru_build::{build_project, generate_toolchain_cmake};
use beru_core::cache::BeruCache;
use beru_core::toolchain;
use beru_manifest::{BeruManifest, Dependency};
use beru_recipe::{beru_exe_dir, fetch_git, resolve_recipe};

/// Arguments for `beru build`.
#[derive(Debug, Args)]
pub struct BuildArgs {
    /// Build profile to use
    #[arg(long, default_value = "debug")]
    pub profile: String,

    /// Optional target filename (e.g., day1.cpp)
    pub target: Option<String>,
}

pub fn exec(args: BuildArgs) -> Result<()> {
    let project_dir = std::env::current_dir().context("failed to get current directory")?;

    let manifest = BeruManifest::from_dir(&project_dir)
        .context("failed to parse Beru.toml (are you in a Beru project directory?)")?;

    let mut resolved_target_name = manifest.package.name.replace('-', "_");

    if manifest.package.package_type == beru_manifest::PackageType::Executable {
        let (target_stem, show_warning) = resolve_target(&project_dir, args.target.as_deref())?;
        resolved_target_name = target_stem.clone();

        if show_warning {
            println!(
                "{} Multiple files found but no 'main.cpp'. Defaulting to '{}.cpp'.",
                style("Warning:").yellow().bold(),
                target_stem
            );
        }

        let cmake_content = format!(
            "cmake_minimum_required(VERSION 3.20)\nproject({} LANGUAGES CXX)\n\nadd_executable({} src/{}.cpp)\n",
            target_stem, target_stem, target_stem
        );
        std::fs::write(project_dir.join("CMakeLists.txt"), cmake_content)
            .context("failed to write dynamic CMakeLists.txt")?;
    }

    println!(
        "{} {} v{} ({})",
        style("Building").green().bold(),
        resolved_target_name,
        manifest.package.version,
        manifest.package.cxx_std,
    );

    let cache = BeruCache::default_location()?;
    cache.ensure_dirs()?;

    let abi_profile = toolchain::build_abi_profile(
        &manifest.package.cxx_std,
        &args.profile,
        manifest.build.shared_libs,
        vec![],
    )?;

    let abi_hash = abi_profile.hash();
    info!("ABI profile: {}", abi_profile);
    debug!("ABI hash: {}", abi_hash);

    let lock_path = project_dir.join("Beru.lock");
    let lockfile = if lock_path.exists() {
        beru_manifest::BeruLock::from_dir(&project_dir).context("Failed to parse Beru.lock")?
    } else {
        info!("Beru.lock not found, resolving dependencies...");
        let beru_exe = std::env::current_exe().ok();
        let generated = beru_resolve::resolve_graph(&manifest, &cache, &project_dir, beru_exe)?;
        std::fs::write(&lock_path, generated.to_string()?).context("Failed to write Beru.lock")?;
        generated
    };

    let mut prefix_paths: Vec<PathBuf> = Vec::new();

    if !lockfile.packages.is_empty() {
        println!("{} dependencies...", style("Building").cyan().bold(),);
    }

    for pkg in &lockfile.packages {
        let opt_dep = manifest.dependencies.get(&pkg.name);

        let install_prefix =
            resolve_and_build_locked_dep(pkg, opt_dep, &cache, &abi_hash, &project_dir)?;
        prefix_paths.push(install_prefix);
    }

    let toolchain_path = project_dir.join("beru-toolchain.cmake");
    let prefix_refs: Vec<&Path> = prefix_paths.iter().map(|p| p.as_path()).collect();
    generate_toolchain_cmake(
        &toolchain_path,
        &manifest.package.cxx_std,
        &args.profile,
        &prefix_refs,
    )?;

    let build_dir = project_dir.join("build");
    println!("{} project...", style("Compiling").green().bold(),);
    build_project(&project_dir, &build_dir, &toolchain_path)?;

    println!(
        "  {} {} built successfully",
        style("Finished").green().bold(),
        resolved_target_name,
    );

    Ok(())
}

/// Resolve a single dependency: find its source, build it, cache the result.
/// Returns the install prefix path for the built dependency.
fn resolve_and_build_locked_dep(
    pkg: &beru_manifest::LockedPackage,
    opt_dep: Option<&Dependency>,
    cache: &BeruCache,
    abi_hash: &str,
    project_dir: &Path,
) -> Result<PathBuf> {
    let name = &pkg.name;
    let version = &pkg.version;
    let install_prefix = cache.build_dir(abi_hash, name, version);

    if cache.has_build(abi_hash, name, version) {
        println!(
            "  {} {} ({}) — cached",
            style("Using").blue().bold(),
            name,
            version,
        );
        return Ok(install_prefix);
    }

    println!(
        "  {} {} ({})",
        style("Fetching").yellow().bold(),
        name,
        pkg.source,
    );

    let source_dir = match opt_dep {
        Some(Dependency::Git(_)) | Some(Dependency::Path(_)) => {
            fetch_dependency_source(name, opt_dep.unwrap(), cache, project_dir)?
        }
        Some(Dependency::Registry(_)) | Some(Dependency::Version(_)) | None => {
            let recipe = resolve_recipe(
                name,
                Some(version),
                project_dir,
                beru_exe_dir().as_deref(),
                Some(&cache.recipes_dir()),
                Some(&cache.index_dir()),
            )?;

            if let Some((r, _)) = recipe {
                let src = r.source;
                if let Some(url) = src.url {
                    if url.ends_with(".tar.gz") {
                        let sha = src.sha256.expect("sha256 missing for tarball");
                        let extracted = beru_recipe::fetch_tarball(cache, &url, &sha)?;
                        beru_recipe::find_source_root(&extracted)?
                    } else {
                        beru_recipe::fetch_git(cache, &url, Some(version))?
                    }
                } else if let Some(git) = src.git {
                    beru_recipe::fetch_git(cache, &git, Some(version))?
                } else {
                    bail!("Recipe for {} has no source", name);
                }
            } else {
                bail!(
                    "Could not find source or recipe for transitive dependency {}",
                    name
                );
            }
        }
    };

    let recipe = resolve_recipe(
        name,
        Some(version),
        project_dir,
        beru_exe_dir().as_deref(),
        Some(&cache.recipes_dir()),
        Some(&cache.index_dir()),
    )?;

    println!("  {} {}...", style("Building").green().bold(), name,);

    std::fs::create_dir_all(&install_prefix)?;

    if let Some((ref r, _)) = recipe {
        if r.build.system == "custom" {
            beru_build::build_dependency_custom(&source_dir, &install_prefix, &r.build.commands)?;
        } else {
            beru_build::build_dependency_cmake(
                &source_dir,
                &install_prefix,
                &r.build.cmake_args,
                None,
            )?;
        }
    } else {
        beru_build::build_dependency_cmake(&source_dir, &install_prefix, &[], None)?;
    }

    Ok(install_prefix)
}

fn fetch_dependency_source(
    name: &str,
    dep: &Dependency,
    cache: &BeruCache,
    project_dir: &Path,
) -> Result<PathBuf> {
    match dep {
        Dependency::Git(g) => {
            let pin = g
                .tag
                .as_deref()
                .or(g.branch.as_deref())
                .or(g.rev.as_deref());
            let repo_dir = fetch_git(cache, &g.git, pin)?;
            Ok(repo_dir)
        }
        Dependency::Path(p) => {
            let resolved = if p.path.is_absolute() {
                p.path.clone()
            } else {
                project_dir.join(&p.path)
            };
            if !resolved.exists() {
                bail!(
                    "path dependency '{}' does not exist: {}",
                    name,
                    resolved.display()
                );
            }
            Ok(resolved)
        }
        Dependency::Registry(_) | Dependency::Version(_) => {
            bail!(
                "Registry dependencies cannot be fetched via fetch_dependency_source. Use recipes instead."
            )
        }
    }
}

/// Resolve the correct target executable file from the src/ directory.
pub fn resolve_target(project_dir: &Path, target_arg: Option<&str>) -> Result<(String, bool)> {
    let src_dir = project_dir.join("src");
    if !src_dir.exists() {
        bail!("src/ directory not found in project");
    }

    let mut cpp_files = Vec::new();
    for entry in std::fs::read_dir(&src_dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_file()
            && path.extension().and_then(|s| s.to_str()) == Some("cpp")
            && let Some(name) = path.file_stem().and_then(|s| s.to_str())
        {
            cpp_files.push(name.to_string());
        }
    }

    if cpp_files.is_empty() {
        bail!("no .cpp files found in src/");
    }

    // Scenario B: User explicitly requested a target
    if let Some(t) = target_arg {
        let stem = t.strip_suffix(".cpp").unwrap_or(t);
        if cpp_files.contains(&stem.to_string()) {
            return Ok((stem.to_string(), false));
        } else {
            bail!("target file '{}.cpp' not found in src/", stem);
        }
    }

    // Scenario A: No arguments
    if cpp_files.len() == 1 {
        // Exactly 1 file
        return Ok((cpp_files[0].clone(), false));
    }

    if cpp_files.contains(&"main".to_string()) {
        // Multiple files, main.cpp exists
        return Ok(("main".to_string(), false));
    }

    // Multiple files, no main.cpp
    cpp_files.sort();
    let default_target = cpp_files[0].clone();

    Ok((default_target, true))
}
