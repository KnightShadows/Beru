use anyhow::Result;
use beru_core::cache::BeruCache;
use beru_manifest::{BeruLock, BeruManifest, LockedPackage};
use pubgrub::resolve;
use pubgrub::SemanticVersion;
use std::path::PathBuf;

/// Compute the full dependency graph for a manifest using PubGrub.
pub fn resolve_graph(
    manifest: &BeruManifest,
    cache: &BeruCache,
    project_dir: &std::path::Path,
    beru_exe_dir: Option<PathBuf>,
) -> Result<BeruLock> {
    let provider = crate::BeruProvider::new(cache, project_dir, beru_exe_dir);

    for (name, dep) in &manifest.dependencies {
        provider.add_source(name, dep);
    }

    let root_pkg = "root".to_string();
    let root_version = SemanticVersion::new(0, 0, 0);

    let mut root_deps = Vec::new();
    for name in manifest.dependencies.keys() {
        root_deps.push((name.clone(), pubgrub::Range::full()));
    }

    provider.deps_cache.borrow_mut().insert(
        (root_pkg.clone(), root_version),
        pubgrub::Dependencies::Available(root_deps.into_iter().collect()),
    );

    let solution = resolve(&provider, root_pkg, root_version)
        .map_err(|e| anyhow::anyhow!("Dependency resolution failed: {}", e))?;

    let mut locked_packages = Vec::new();
    for (name, version) in solution {
        if name == "root" {
            continue;
        }

        let source_str = if let Some(dep) = provider.sources.borrow().get(&name) {
            dep.source_display()
        } else {
            "bundled".to_string()
        };

        let deps_cache = provider.deps_cache.borrow();
        let deps_keys: Vec<String> = if let Some(pubgrub::Dependencies::Available(deps)) =
            deps_cache.get(&(name.clone(), version))
        {
            deps.iter().map(|(k, _)| k.clone()).collect()
        } else {
            Vec::new()
        };

        locked_packages.push(LockedPackage {
            name,
            version: version.to_string(),
            source: source_str,
            checksum: None,
            dependencies: deps_keys,
        });
    }

    locked_packages.sort_by(|a, b| a.name.cmp(&b.name));

    Ok(BeruLock {
        version: 1,
        packages: locked_packages,
    })
}
