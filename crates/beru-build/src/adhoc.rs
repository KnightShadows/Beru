use crate::{CMakeDependency, build_project, generate_toolchain_cmake};
use anyhow::{Result, bail};
use beru_core::cache::BeruCache;
use beru_manifest::{BeruManifest, Dependency, LockedPackage};
use sha2::{Digest, Sha256};
use std::path::{Path, PathBuf};

/// Build (and cache) an ad-hoc single-file script as an isolated CMake project.
/// Never touches any file outside `cache_dir` and the returned build directory within it.
pub fn build_adhoc(
    entry_file: &Path,
    manifest: &BeruManifest,
    profile: &str,
    cache: &BeruCache,
) -> Result<PathBuf> {
    let entry_file_contents = std::fs::read_to_string(entry_file)?;

    let abi_profile = beru_core::toolchain::build_abi_profile(
        &manifest.package.cxx_std,
        profile,
        manifest.build.shared_libs,
        vec![],
    )?;

    let project_dir = cache.adhoc_dir();
    let lockfile =
        beru_resolve::resolve_graph(manifest, cache, &project_dir, std::env::current_exe().ok())?;

    let mut hasher = Sha256::new();
    hasher.update(b"abi:");
    hasher.update(abi_profile.hash().as_bytes());
    hasher.update(b"\nsource:");
    hasher.update(entry_file_contents.as_bytes());
    hasher.update(b"\ndeps:");
    for pkg in &lockfile.packages {
        hasher.update(pkg.name.as_bytes());
        hasher.update(b"@");
        hasher.update(pkg.version.as_bytes());
    }
    let hash = hasher
        .finalize()
        .iter()
        .map(|b| format!("{:02x}", b))
        .collect::<String>();

    let build_dir = cache.adhoc_build_dir(&hash);
    let binary_name = entry_file.file_stem().unwrap().to_string_lossy();
    let binary_path = build_dir.join(if cfg!(windows) {
        format!("{}.exe", binary_name)
    } else {
        binary_name.to_string()
    });

    if binary_path.exists() {
        println!("  cache hit");
        return Ok(binary_path);
    }

    let abi_hash = abi_profile.hash();
    let mut prefix_paths = Vec::new();
    let mut cmake_deps = Vec::new();

    for pkg in &lockfile.packages {
        let opt_dep = manifest.dependencies.get(&pkg.name);

        let install_prefix = build_locked_dep(pkg, opt_dep, cache, &abi_hash, &project_dir)?;
        prefix_paths.push(install_prefix);

        let recipe = beru_recipe::resolve_recipe(
            &pkg.name,
            Some(&pkg.version),
            &project_dir,
            std::env::current_exe()
                .ok()
                .as_deref()
                .and_then(|p| p.parent()),
            Some(&cache.recipes_dir()),
            Some(&cache.index_dir()),
        )?;

        if let Some((r, _)) = recipe {
            let mut targets = r.export.cmake_targets.clone();
            if targets.is_empty() {
                targets = r.export.link_libs.clone();
            }
            cmake_deps.push(CMakeDependency {
                package_name: r.export.cmake_package.clone(),
                targets,
            });
        }
    }

    std::fs::create_dir_all(&build_dir)?;

    let target_links = cmake_deps
        .iter()
        .flat_map(|d| d.targets.iter())
        .map(|s| s.as_str())
        .collect::<Vec<_>>()
        .join(" ");

    let link_command = if !target_links.is_empty() {
        format!(
            "target_link_libraries({} PRIVATE {})",
            binary_name, target_links
        )
    } else {
        String::new()
    };

    let absolute_entry = entry_file
        .canonicalize()
        .unwrap_or_else(|_| entry_file.to_path_buf());
    let mut absolute_entry_str = absolute_entry.to_string_lossy().into_owned();
    if absolute_entry_str.starts_with("\\\\?\\") {
        absolute_entry_str = absolute_entry_str[4..].to_string();
    }
    let absolute_entry_str = absolute_entry_str.replace("\\", "/");

    let cmakelists = format!(
        "cmake_minimum_required(VERSION 3.20)\n\
        project(adhoc-script)\n\
        set(CMAKE_RUNTIME_OUTPUT_DIRECTORY \"${{CMAKE_BINARY_DIR}}\")\n\
        set(CMAKE_RUNTIME_OUTPUT_DIRECTORY_DEBUG \"${{CMAKE_BINARY_DIR}}\")\n\
        set(CMAKE_RUNTIME_OUTPUT_DIRECTORY_RELEASE \"${{CMAKE_BINARY_DIR}}\")\n\
        set(CMAKE_RUNTIME_OUTPUT_DIRECTORY_RELWITHDEBINFO \"${{CMAKE_BINARY_DIR}}\")\n\
        set(CMAKE_RUNTIME_OUTPUT_DIRECTORY_MINSIZEREL \"${{CMAKE_BINARY_DIR}}\")\n\
        add_executable({} \"{}\")\n{}\n",
        binary_name, absolute_entry_str, link_command
    );
    std::fs::write(build_dir.join("CMakeLists.txt"), cmakelists)?;

    let toolchain_file = build_dir.join("beru-toolchain.cmake");
    let prefix_refs: Vec<&Path> = prefix_paths.iter().map(|p| p.as_path()).collect();
    generate_toolchain_cmake(
        &toolchain_file,
        &manifest.package.cxx_std,
        profile,
        &prefix_refs,
        &cmake_deps,
    )?;

    build_project(&build_dir, &build_dir, &toolchain_file, None, &[])?;

    Ok(binary_path)
}

fn build_locked_dep(
    pkg: &LockedPackage,
    opt_dep: Option<&Dependency>,
    cache: &BeruCache,
    abi_hash: &str,
    project_dir: &Path,
) -> Result<PathBuf> {
    let name = &pkg.name;
    let version = &pkg.version;
    let install_prefix = cache.build_dir(abi_hash, name, version);

    if cache.has_build(abi_hash, name, version) {
        return Ok(install_prefix);
    }

    let source_dir = match opt_dep {
        Some(Dependency::Git(g)) => {
            let pin = pkg
                .checksum
                .as_deref()
                .or(g.rev.as_deref())
                .or(g.tag.as_deref())
                .or(g.branch.as_deref());
            beru_recipe::fetch_git(cache, &g.git, pin)?
        }
        Some(Dependency::Path(p)) => {
            let resolved = if p.path.is_absolute() {
                p.path.clone()
            } else {
                project_dir.join(&p.path)
            };
            if !resolved.exists() {
                bail!("path dependency not found");
            }
            resolved
        }
        _ => {
            let recipe = beru_recipe::resolve_recipe(
                name,
                Some(version),
                project_dir,
                std::env::current_exe()
                    .ok()
                    .as_deref()
                    .and_then(|p| p.parent()),
                Some(&cache.recipes_dir()),
                Some(&cache.index_dir()),
            )?;
            if let Some((r, _)) = recipe {
                let src = r.source;
                if let Some(url) = src.url {
                    if url.ends_with(".tar.gz") {
                        let sha = pkg.checksum.clone().or(src.sha256).expect("sha256 missing");
                        let extracted = beru_recipe::fetch_tarball(cache, &url, &sha)?;
                        beru_recipe::find_source_root(&extracted)?
                    } else {
                        let pin = pkg
                            .checksum
                            .as_deref()
                            .or(src.tag.as_deref())
                            .or(Some(version));
                        beru_recipe::fetch_git(cache, &url, pin)?
                    }
                } else if let Some(git) = src.git {
                    let pin = pkg
                        .checksum
                        .as_deref()
                        .or(src.tag.as_deref())
                        .or(Some(version));
                    beru_recipe::fetch_git(cache, &git, pin)?
                } else {
                    bail!("Recipe for {} has no source", name);
                }
            } else {
                bail!("Could not find source or recipe for {}", name);
            }
        }
    };

    let recipe = beru_recipe::resolve_recipe(
        name,
        Some(version),
        project_dir,
        std::env::current_exe()
            .ok()
            .as_deref()
            .and_then(|p| p.parent()),
        Some(&cache.recipes_dir()),
        Some(&cache.index_dir()),
    )?;

    std::fs::create_dir_all(&install_prefix)?;

    if let Some((r, _)) = recipe {
        if r.build.system == "custom" {
            crate::build_dependency_custom(&source_dir, &install_prefix, &r.build.commands)?;
        } else {
            crate::build_dependency_cmake(&source_dir, &install_prefix, &r.build.cmake_args, None)?;
        }
    } else {
        crate::build_dependency_cmake(&source_dir, &install_prefix, &[], None)?;
    }

    Ok(install_prefix)
}
