use anyhow::{Context, Result, bail};
use clap::Args;
use console::style;
use std::fs;
use toml_edit::{DocumentMut, Item, Table, value};

/// Arguments for `beru add`.
#[derive(Debug, Args)]
pub struct AddArgs {
    /// The name of the package to add (e.g. `fmt@11.0.2` or just `fmt`).
    pub package: String,
}

pub fn exec(args: AddArgs) -> Result<()> {
    let project_dir = std::env::current_dir().context("failed to get current directory")?;
    let manifest_path = project_dir.join("Beru.toml");

    if !manifest_path.exists() {
        bail!("No Beru.toml found in the current directory.");
    }

    // Parse the package string, e.g., "fmt@11.0.2" -> name: "fmt", version: "11.0.2"
    let (name, version) = if let Some(idx) = args.package.find('@') {
        let n = &args.package[..idx];
        let v = &args.package[idx + 1..];
        (n, v)
    } else {
        // If no version specified, default to "*" or a known format.
        // For C++, explicit versions are highly recommended, but we can default to "*".
        (args.package.as_str(), "*")
    };

    // Read the file content
    let content = fs::read_to_string(&manifest_path).context("failed to read Beru.toml")?;

    // Parse it into a mutable AST
    let mut doc = content
        .parse::<DocumentMut>()
        .context("failed to parse Beru.toml as AST")?;

    // Ensure [dependencies] table exists
    if !doc.contains_key("dependencies") {
        doc["dependencies"] = Item::Table(Table::new());
    }

    let deps = doc["dependencies"]
        .as_table_mut()
        .context("[dependencies] must be a table")?;

    // Add or update the dependency
    deps.insert(name, value(version));

    // Write it back to disk
    fs::write(&manifest_path, doc.to_string()).context("failed to write Beru.toml")?;

    println!(
        "{} dependency {} v{} to Beru.toml",
        style("Added").green().bold(),
        style(name).cyan().bold(),
        style(version).yellow()
    );

    Ok(())
}
