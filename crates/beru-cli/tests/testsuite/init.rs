use crate::support::project;
use predicates::prelude::*;

#[test]
fn test_beru_init_library() {
    let p = project("init-proj").build();

    p.beru("init")
        .arg("--type")
        .arg("library")
        .assert()
        .success()
        .stdout(predicate::str::contains("Initializing Beru"));

    assert!(p.root().join("Beru.toml").exists());
    assert!(p.root().join("src").join("init-proj.cpp").exists());
}
