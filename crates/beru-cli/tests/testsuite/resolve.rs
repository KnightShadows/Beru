use crate::support::project;
use std::fs;

#[test]
fn test_beru_resolve_dependencies() {
    let p = project("resolve-proj")
        .file(
            "Beru.toml",
            r#"
            [package]
            name = "resolve-proj"
            version = "0.1.0"

            [dependencies]
            fmt = "11.0.2"
            "#,
        )
        .build();

    // First update the index in our isolated sandbox
    p.beru("index").arg("update").assert().success();

    // Now resolve dependencies
    p.beru("resolve").assert().success();

    let lockfile = fs::read_to_string(p.root().join("Beru.lock")).expect("failed to read lockfile");
    assert!(lockfile.contains("fmt"));
    assert!(lockfile.contains("11.0.2"));
}
