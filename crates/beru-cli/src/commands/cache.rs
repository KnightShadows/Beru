use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use console::style;
use std::path::{Path, PathBuf};

use beru_core::cache::BeruCache;

/// Arguments for `beru cache`.
#[derive(Debug, Parser)]
pub struct CacheArgs {
    #[command(subcommand)]
    pub command: CacheCommand,
}

/// `beru cache` subcommands.
#[derive(Debug, Subcommand)]
pub enum CacheCommand {
    /// Show disk usage for each cache category
    Size,
    /// Remove cached data. With no flags, cleans every category below.
    Clean {
        /// Remove only downloaded sources (tarballs + git clones)
        #[arg(long)]
        sources: bool,
        /// Remove only compiled dependency artifacts
        #[arg(long)]
        builds: bool,
        /// Remove only the ad-hoc script build cache
        #[arg(long)]
        adhoc: bool,
    },
}

pub fn exec(args: CacheArgs) -> Result<()> {
    let cache = BeruCache::default_location()?;
    cache.ensure_dirs()?;
    match args.command {
        CacheCommand::Size => exec_size(&cache),
        CacheCommand::Clean {
            sources,
            builds,
            adhoc,
        } => exec_clean(&cache, sources, builds, adhoc),
    }
}

fn exec_clean(cache: &BeruCache, sources: bool, builds: bool, adhoc: bool) -> Result<()> {
    let clean_all = !sources && !builds && !adhoc;

    let mut targets: Vec<(&str, PathBuf)> = Vec::new();
    if clean_all || sources {
        targets.push(("sources", cache.sources_dir()));
        targets.push(("git clones", cache.git_dir()));
    }
    if clean_all || builds {
        targets.push(("builds", cache.builds_dir()));
    }
    if clean_all || adhoc {
        targets.push(("adhoc", cache.adhoc_dir()));
    }

    let mut cleaned_any = false;
    for (label, dir) in &targets {
        if dir.exists() {
            std::fs::remove_dir_all(dir)
                .with_context(|| format!("failed to remove {}", dir.display()))?;
            println!("  {} {}", style("Removed").green().bold(), label);
            cleaned_any = true;
        }
    }

    cache.ensure_dirs()?;

    if cleaned_any {
        println!("{} cache", style("Cleaned").green().bold());
    } else {
        println!("{} Nothing to clean", style("Skipped").yellow().bold());
    }

    Ok(())
}

fn exec_size(cache: &BeruCache) -> Result<()> {
    let categories: [(&str, PathBuf); 3] = [
        ("sources", cache.sources_dir()),
        ("builds", cache.builds_dir()),
        ("adhoc", cache.adhoc_dir()),
    ];

    let mut total = 0u64;
    for (label, dir) in &categories {
        let size = dir_size(dir);
        total += size;
        println!("  {:<10} {}", label, human_size(size));
    }
    println!("  {:<10} {}", "total", human_size(total));
    Ok(())
}

/// Recursive directory size, matching the existing hand-rolled recursion style already used by
/// `find_file_recursive` in `commands/run.rs` — no new dependency for this.
fn dir_size(dir: &Path) -> u64 {
    let Ok(entries) = std::fs::read_dir(dir) else {
        return 0;
    };
    let mut total = 0u64;
    for entry in entries.flatten() {
        let path = entry.path();
        if let Ok(meta) = entry.metadata() {
            total += if meta.is_dir() {
                dir_size(&path)
            } else {
                meta.len()
            };
        }
    }
    total
}

fn human_size(bytes: u64) -> String {
    const UNITS: [&str; 5] = ["B", "KB", "MB", "GB", "TB"];
    let mut size = bytes as f64;
    let mut unit = 0;
    while size >= 1024.0 && unit < UNITS.len() - 1 {
        size /= 1024.0;
        unit += 1;
    }
    if unit == 0 {
        format!("{bytes} B")
    } else {
        format!("{size:.1} {}", UNITS[unit])
    }
}
