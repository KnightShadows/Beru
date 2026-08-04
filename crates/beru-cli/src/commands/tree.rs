use anyhow::{Context, Result, bail};
use beru_manifest::{BeruLock, BeruManifest, LockedPackage};
use clap::Args;
use std::collections::{HashMap, HashSet};
use termtree::Tree;

/// Arguments for `beru tree`.
#[derive(Debug, Args)]
pub struct TreeArgs {}

pub fn exec(_args: TreeArgs) -> Result<()> {
    let project_dir = std::env::current_dir().context("failed to get current directory")?;

    let manifest = BeruManifest::from_dir(&project_dir).context("failed to parse Beru.toml")?;
    let lockfile_path = project_dir.join("Beru.lock");

    if !lockfile_path.exists() {
        bail!("No Beru.lock found. Run `beru resolve` or `beru build` first.");
    }

    let lockfile = BeruLock::from_dir(&project_dir).context("failed to parse Beru.lock")?;

    // Build a map of package name -> locked package
    let locked_packages: HashMap<&str, &LockedPackage> = lockfile
        .packages
        .iter()
        .map(|p| (p.name.as_str(), p))
        .collect();

    let mut visited = HashSet::new();

    let root_node = format!("{} v{}", manifest.package.name, manifest.package.version);
    let mut tree = Tree::new(root_node);

    for dep_name in manifest.dependencies.keys() {
        if let Some(locked_pkg) = locked_packages.get(dep_name.as_str()) {
            tree.push(build_tree_node(locked_pkg, &locked_packages, &mut visited));
        } else {
            tree.push(Tree::new(format!("{} (unresolved)", dep_name)));
        }
    }

    println!("{}", tree);

    Ok(())
}

fn build_tree_node(
    pkg: &LockedPackage,
    packages: &HashMap<&str, &LockedPackage>,
    visited: &mut HashSet<String>,
) -> Tree<String> {
    let label = format!("{} v{}", pkg.name, pkg.version);

    if visited.contains(&pkg.name) {
        // Break cycles or redundant subtrees
        return Tree::new(format!("{} (*)", label));
    }

    visited.insert(pkg.name.clone());

    let mut tree = Tree::new(label);

    for dep_name in &pkg.dependencies {
        if let Some(locked_dep) = packages.get(dep_name.as_str()) {
            tree.push(build_tree_node(locked_dep, packages, visited));
        } else {
            tree.push(Tree::new(format!("{} (unresolved)", dep_name)));
        }
    }

    tree
}
