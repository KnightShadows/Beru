use crate::support::project;
use predicates::prelude::*;

#[test]
fn test_beru_run_project() {
    let p = project("run-proj").build();

    p.beru("init")
        .arg("--type")
        .arg("executable")
        .assert()
        .success();

    p.beru("run")
        .assert()
        .success()
        .stdout(predicate::str::contains("Hello from Beru!"));
}

#[test]
fn test_beru_run_adhoc_script() {
    let p = project("run-script")
        .file(
            "Beru.toml",
            r#"
            [package]
            name = "run-script"
            version = "0.1.0"
            type = "executable"
            "#,
        )
        .file(
            "src/script.cpp",
            r#"
            #include <iostream>
            int main() {
                std::cout << "Ad-hoc Script Running!" << std::endl;
                return 0;
            }
            "#,
        )
        .build();

    p.beru("init")
        .arg("--type")
        .arg("executable")
        .assert()
        .success();

    p.beru("run")
        .arg("src/script.cpp")
        .assert()
        .success()
        .stdout(predicate::str::contains("Ad-hoc Script Running!"));
}
