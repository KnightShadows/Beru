use anyhow::{Context, Result, bail};
use clap::Args;
use console::style;
use std::process::Command;

use beru_manifest::BeruManifest;

/// Arguments for `beru run`.
#[derive(Debug, Args)]
pub struct RunArgs {
    /// Build profile to use
    #[arg(long, default_value = "debug")]
    pub profile: String,

    /// Arguments to pass to the executable
    #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
    pub args: Vec<String>,
}

pub fn exec(args: RunArgs) -> Result<()> {
    let project_dir = std::env::current_dir().context("failed to get current directory")?;

    let manifest = BeruManifest::from_dir(&project_dir).context("failed to parse Beru.toml")?;

    if manifest.package.package_type != beru_manifest::PackageType::Executable {
        bail!(
            "`beru run` is only for executable projects. This project is type '{}'.",
            manifest.package.package_type
        );
    }

    let mut target = None;
    let mut run_args = args.args.clone();

    if let Some(first) = run_args.first() {
        let stem = first.strip_suffix(".cpp").unwrap_or(first);
        let src_file = project_dir.join("src").join(format!("{}.cpp", stem));
        if src_file.exists() {
            target = Some(stem.to_string());
            run_args.remove(0); // Pop the target off so it doesn't get passed as an arg to the executable
        }
    }

    let (resolved_target, _) = super::build::resolve_target(&project_dir, target.as_deref())?;

    let build_args = super::build::BuildArgs {
        profile: args.profile.clone(),
        target: target.clone(),
    };
    super::build::exec(build_args)?;

    let build_dir = project_dir.join("build");

    let exe_path = find_executable(&build_dir, &resolved_target)?;

    println!(
        "  {} `{}`\n",
        style("Running").green().bold(),
        resolved_target,
    );

    let status = Command::new(&exe_path)
        .args(&run_args)
        .status()
        .with_context(|| format!("failed to run {}", exe_path.display()))?;

    if !status.success() {
        std::process::exit(status.code().unwrap_or(1));
    }

    Ok(())
}

/// Search for the built executable in common CMake output locations.
fn find_executable(build_dir: &std::path::Path, name: &str) -> Result<std::path::PathBuf> {
    let candidates = [
        build_dir.join(name),
        build_dir.join(format!("{name}.exe")),
        build_dir.join("Debug").join(name),
        build_dir.join("Release").join(name),
    ];

    for candidate in &candidates {
        if candidate.exists() {
            return Ok(candidate.clone());
        }
    }

    if let Some(found) = find_file_recursive(build_dir, name) {
        return Ok(found);
    }

    bail!(
        "could not find executable '{}' in {}. Was the build successful?",
        name,
        build_dir.display()
    )
}

/// Recursively search for a file by name in a directory.
fn find_file_recursive(dir: &std::path::Path, name: &str) -> Option<std::path::PathBuf> {
    let entries = std::fs::read_dir(dir).ok()?;
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_file() {
            if let Some(file_name) = path.file_name().and_then(|n| n.to_str())
                && (file_name == name || file_name == format!("{name}.exe"))
            {
                return Some(path);
            }
        } else if path.is_dir()
            && let Some(found) = find_file_recursive(&path, name)
        {
            return Some(found);
        }
    }
    None
}
