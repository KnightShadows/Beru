use crate::support::project;
use predicates::prelude::*;

#[test]
fn test_beru_build_success() {
    let p = project("build-proj").build();

    p.beru("init")
        .arg("--type")
        .arg("executable")
        .assert()
        .success();

    p.beru("build")
        .assert()
        .success()
        .stdout(predicate::str::contains("built successfully"));
}
