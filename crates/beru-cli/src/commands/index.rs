use anyhow::{Context, Result, bail};
use beru_core::cache::BeruCache;
use clap::{Parser, Subcommand};
use std::process::Command;
use tracing::info;

#[derive(Debug, Parser)]
pub struct IndexArgs {
    #[command(subcommand)]
    pub command: IndexCommand,
}

#[derive(Debug, Subcommand)]
pub enum IndexCommand {
    /// Update the global package registry index
    Update {
        /// Optional URL to a custom Git registry
        #[arg(long)]
        url: Option<String>,
    },
}

pub fn exec(args: IndexArgs) -> Result<()> {
    match args.command {
        IndexCommand::Update { url } => {
            let cache = BeruCache::default_location()?;
            cache.ensure_dirs()?;

            let index_dir = cache.index_dir();

            let default_url = "https://github.com/KnightShadows/beru_index.git";
            let url = url.as_deref().unwrap_or(default_url);

            info!("Updating Beru index from {}", url);

            if index_dir.join(".git").exists() {
                let output = Command::new("git")
                    .args(["pull", "--ff-only", url])
                    .current_dir(&index_dir)
                    .output()
                    .context("Failed to pull index repository")?;

                if !output.status.success() {
                    let stderr = String::from_utf8_lossy(&output.stderr);
                    bail!("Failed to update index:\n{}", stderr);
                }
            } else {
                if index_dir.exists() {
                    std::fs::remove_dir_all(&index_dir)?;
                }

                let output = Command::new("git")
                    .args(["clone", "--depth", "1", url])
                    .arg(&index_dir)
                    .output()
                    .context("Failed to clone index repository")?;

                if !output.status.success() {
                    let stderr = String::from_utf8_lossy(&output.stderr);
                    bail!("Failed to clone index:\n{}", stderr);
                }
            }

            info!("Index successfully updated at {}", index_dir.display());
        }
    }

    Ok(())
}
