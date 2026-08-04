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

    if let Some(first) = run_args.first().cloned() {
        let path = std::path::Path::new(&first);
        if path.extension().and_then(|s| s.to_str()) == Some("cpp") && path.exists() {
            // It's an ad-hoc file
            let stem = path.file_stem().unwrap().to_string_lossy().into_owned();
            target = Some(stem.clone());
            run_args.remove(0);

            // Auto-edit CMakeLists.txt if target is missing
            auto_append_target(&project_dir, &stem, path)?;
        } else {
            let stem = first.strip_suffix(".cpp").unwrap_or(&first);
            let src_file = project_dir.join("src").join(format!("{}.cpp", stem));
            if src_file.exists() {
                target = Some(stem.to_string());
                run_args.remove(0); // Pop the target off so it doesn't get passed as an arg to the executable

                // Auto-edit CMakeLists.txt if target is missing
                let rel_path = std::path::Path::new("src").join(format!("{}.cpp", stem));
                auto_append_target(&project_dir, stem, &rel_path)?;
            }
        }
    }

    let (resolved_target, _) = super::build::resolve_target(&project_dir, target.as_deref())?;
    let mut actual_target_name = resolved_target.clone();
    if resolved_target == "main" {
        actual_target_name = manifest.package.name.clone();
    }

    let build_args = super::build::BuildArgs {
        profile: args.profile.clone(),
        target: target.clone(),
    };
    super::build::exec(build_args)?;

    let build_dir = project_dir.join("build");

    let exe_path = find_executable(&build_dir, &actual_target_name)?;

    println!(
        "  {} `{}`\n",
        style("Running").green().bold(),
        actual_target_name,
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

/// Automatically appends an executable target to CMakeLists.txt if it does not already exist.
fn auto_append_target(
    project_dir: &std::path::Path,
    target_name: &str,
    cpp_file: &std::path::Path,
) -> Result<()> {
    let cmakelists = project_dir.join("CMakeLists.txt");
    if !cmakelists.exists() {
        return Ok(());
    }

    let content = std::fs::read_to_string(&cmakelists)?;

    // Check if the target is already defined
    let search1 = format!("add_executable({}", target_name);
    let search2 = format!("add_executable( {}", target_name);

    if !content.contains(&search1) && !content.contains(&search2) {
        let mut new_content = content.clone();
        if !new_content.ends_with('\n') {
            new_content.push('\n');
        }

        // Ensure path uses forward slashes for CMake
        let path_str = cpp_file.display().to_string().replace("\\", "/");

        new_content.push_str("\n# --- Auto-generated by Beru ---\n");
        new_content.push_str(&format!("add_executable({} {})\n", target_name, path_str));
        new_content.push_str(&format!("beru_link_dependencies({})\n", target_name));

        std::fs::write(&cmakelists, new_content).context("Failed to auto-update CMakeLists.txt")?;

        println!(
            "{} target '{}' to CMakeLists.txt",
            style("Auto-added").green().bold(),
            target_name
        );
    }

    Ok(())
}
