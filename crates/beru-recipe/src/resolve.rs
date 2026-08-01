use anyhow::{Context, Result};
use std::path::{Path, PathBuf};
use tracing::debug;

use crate::Recipe;

/// The location where bundled recipes live (inside the Beru binary's crate).
/// At runtime, these are read from the `recipes/` directory relative to the
/// binary, or embedded as a fallback.
const BUNDLED_RECIPES_DIR: &str = "recipes";

/// Resolve a recipe for a given package name.
///
/// Resolution order:
/// 1. Project-local `recipes/<name>/recipe.toml`
/// 2. Bundled recipes shipped with Beru
/// 3. User's global recipes at `~/.beru/recipes/<name>/recipe.toml`
///
/// Returns `None` if no recipe is found.
pub fn resolve_recipe(
    package_name: &str,
    version: Option<&str>,
    project_dir: &Path,
    beru_exe_dir: Option<&Path>,
    global_recipes_dir: Option<&Path>,
    index_dir: Option<&Path>,
) -> Result<Option<(Recipe, PathBuf)>> {
    let local_recipe = project_dir
        .join("recipes")
        .join(package_name)
        .join("recipe.toml");
    if local_recipe.exists() {
        debug!("found project-local recipe at {}", local_recipe.display());
        let recipe = Recipe::from_file(&local_recipe)
            .with_context(|| format!("failed to parse {}", local_recipe.display()))?;
        return Ok(Some((recipe, local_recipe)));
    }

    if let Some(exe_dir) = beru_exe_dir {
        let bundled_recipe = exe_dir
            .join(BUNDLED_RECIPES_DIR)
            .join(package_name)
            .join("recipe.toml");
        if bundled_recipe.exists() {
            debug!("found bundled recipe at {}", bundled_recipe.display());
            let recipe = Recipe::from_file(&bundled_recipe)
                .with_context(|| format!("failed to parse {}", bundled_recipe.display()))?;
            return Ok(Some((recipe, bundled_recipe)));
        }
    }

    if let Some(global_dir) = global_recipes_dir {
        let global_recipe = global_dir.join(package_name).join("recipe.toml");
        if global_recipe.exists() {
            debug!("found global recipe at {}", global_recipe.display());
            let recipe = Recipe::from_file(&global_recipe)
                .with_context(|| format!("failed to parse {}", global_recipe.display()))?;
            return Ok(Some((recipe, global_recipe)));
        }
    }

    if let (Some(idx), Some(ver)) = (index_dir, version) {
        let index_recipe = idx.join(package_name).join(ver).join("recipe.toml");
        if index_recipe.exists() {
            debug!("found index recipe at {}", index_recipe.display());
            let recipe = Recipe::from_file(&index_recipe)
                .with_context(|| format!("failed to parse {}", index_recipe.display()))?;
            return Ok(Some((recipe, index_recipe)));
        }
    }

    Ok(None)
}

/// Find the Beru executable's directory (for locating bundled recipes).
pub fn beru_exe_dir() -> Option<PathBuf> {
    std::env::current_exe()
        .ok()
        .and_then(|p| p.parent().map(|p| p.to_path_buf()))
}
