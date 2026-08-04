use crate::support::project;
use predicates::prelude::*;
use std::fs;

#[test]
fn test_beru_test_success() {
    let p = project("test-proj").build();

    p.beru("init")
        .arg("--type")
        .arg("executable")
        .assert()
        .success();

    // Since our init currently creates a basic CMakeLists.txt that just has an executable,
    // let's manually write a CMakeLists.txt that includes testing
    let cmakelists_path = p.root().join("CMakeLists.txt");
    let cmake_content = r#"
cmake_minimum_required(VERSION 3.20)
project(test-proj LANGUAGES CXX)

enable_testing()

add_executable(test-proj src/main.cpp)
add_test(NAME MyTest COMMAND test-proj)
"#;
    fs::write(cmakelists_path, cmake_content).unwrap();

    // We must run beru resolve to set up Beru.lock, but beru test calls beru build which handles it
    p.beru("test")
        .assert()
        .success()
        .stdout(predicate::str::contains("Running tests..."))
        .stdout(predicate::str::contains("100% tests passed"));
}
