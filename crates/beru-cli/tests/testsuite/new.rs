use crate::support::project;
use predicates::prelude::*;

#[test]
fn test_beru_new_executable() {
    let p = project("new-proj").build();

    p.beru("new")
        .arg("test-proj")
        .arg("--type")
        .arg("executable")
        .assert()
        .success()
        .stdout(predicate::str::contains("Creating executable `test-proj`"));

    let proj_dir = p.root().join("test-proj");
    assert!(proj_dir.join("Beru.toml").exists());
    assert!(proj_dir.join("src").join("main.cpp").exists());
}
