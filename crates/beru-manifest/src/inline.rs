use crate::{BeruManifest, ManifestError};

/// Extracts an embedded manifest from a `// /// beru` ... `// ///` comment fence at the top of a
/// source file. Returns `None` if no such block exists — this is not an error, it's the
/// zero-dependency case.
pub fn extract_inline_manifest(source: &str) -> Result<Option<BeruManifest>, ManifestError> {
    const START: &str = "/// beru";
    const END: &str = "///";

    // Find the start fence. It must appear on its own comment line, ignoring leading whitespace
    // and a leading "//". Do not accept it mid-line — that invites ambiguity with normal comments.
    let Some(start_idx) = source.lines().position(|l| l.trim_start_matches("//").trim() == START)
    else {
        return Ok(None);
    };

    let mut toml_lines = Vec::new();
    let mut found_end = false;
    for line in source.lines().skip(start_idx + 1) {
        let trimmed = line.trim_start();
        let stripped = trimmed.trim_start_matches("//").trim_start();
        if stripped.trim_end() == END {
            found_end = true;
            break;
        }
        // Strip exactly one leading "// " (or "//")
        toml_lines.push(trimmed.trim_start_matches("// ").trim_start_matches("//"));
    }

    if !found_end {
        return Err(ManifestError::Parse(
            "unterminated inline `/// beru` manifest block (missing closing `///`)".into(),
        ));
    }

    let toml_body = toml_lines.join("\n");
    let toml_body = ensure_default_package(&toml_body);
    BeruManifest::parse_toml(&toml_body).map(Some)
}

/// `BeruManifest::validate()` requires `[package].name` and a valid semver `[package].version`.
/// An inline script manifest that only lists `[dependencies]` must not be forced to redundantly
/// restate package metadata it doesn't need. Inject a default `[package]` table if one is absent.
fn ensure_default_package(toml_body: &str) -> String {
    if toml_body.contains("[package]") {
        return toml_body.to_string();
    }
    format!(
        "[package]\nname = \"adhoc-script\"\nversion = \"0.0.0\"\ntype = \"executable\"\n\n{}",
        toml_body
    )
}

/// Returns a default manifest for ad-hoc scripts that have no inline block and no surrounding project.
pub fn default_adhoc_manifest() -> BeruManifest {
    BeruManifest::parse_toml("[package]\nname = \"adhoc-script\"\nversion = \"0.0.0\"\ntype = \"executable\"\n").unwrap()
}
