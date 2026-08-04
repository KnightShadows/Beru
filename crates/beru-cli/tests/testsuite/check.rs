use crate::support::project;
use predicates::prelude::*;

#[test]
fn test_beru_check_success() {
    let p = project("check-proj").build();

    p.beru("init")
        .arg("--type")
        .arg("executable")
        .assert()
        .success();

    p.beru("check")
        .assert()
        .success()
        .stdout(predicate::str::contains("[syntax-only]"))
        .stdout(predicate::str::contains("checked successfully"));
}
