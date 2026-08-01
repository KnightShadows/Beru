use anyhow::{Context, Result};
use clap::Args;
use console::style;

/// Arguments for `beru init`.
#[derive(Debug, Args)]
pub struct InitArgs {
    /// Project type
    #[arg(long, default_value = "executable", value_parser = ["executable", "library", "header-only"])]
    pub r#type: String,

    /// C++ standard to use
    #[arg(long, default_value = "c++17")]
    pub cxx_std: String,
}

pub fn exec(args: InitArgs) -> Result<()> {
    let cwd = std::env::current_dir().context("failed to get current directory")?;
    let name = cwd
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("my-project")
        .to_string();

    if cwd.join("Beru.toml").exists() {
        println!(
            "{} Beru.toml already exists in this directory.",
            style("Warning:").yellow().bold()
        );
        return Ok(());
    }

    println!(
        "{} Beru in existing directory ({})",
        style("Initializing").green().bold(),
        name,
    );

    let manifest = format!(
        r#"[package]
name = "{name}"
version = "0.1.0"
cxx-std = "{cxx_std}"
type = "{pkg_type}"

[dependencies]

[dev-dependencies]

[build]
system = "cmake"
"#,
        name = name,
        cxx_std = args.cxx_std,
        pkg_type = args.r#type,
    );

    std::fs::write(cwd.join("Beru.toml"), manifest).context("failed to write Beru.toml")?;

    println!(
        "{} Beru.toml created. Run `beru build` to build your project.",
        style("Done:").green().bold()
    );

    Ok(())
}
