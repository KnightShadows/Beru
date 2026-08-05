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

#[test]
fn test_beru_run_root_adhoc_script_no_name_collision() {
    let p = project("hello")
        .file(
            "Beru.toml",
            r#"
            [package]
            name = "hello"
            version = "0.1.0"
            type = "executable"
            cxx-std = "c++17"
            "#,
        )
        .file(
            "hello.cpp",
            r#"
            #include <iostream>
            int main() {
                std::cout << "Root Ad-hoc Script Running!" << std::endl;
                return 0;
            }
            "#,
        )
        .build();

    let cmakelists = p.root().join("CMakeLists.txt");
    std::fs::write(&cmakelists, "cmake_minimum_required(VERSION 3.20)\nproject(hello)\n").unwrap();
    let original_content = std::fs::read_to_string(&cmakelists).unwrap();

    p.beru("run")
        .arg("hello.cpp")
        .assert()
        .success()
        .stdout(predicate::str::contains("Root Ad-hoc Script Running!"));

    let new_content = std::fs::read_to_string(&cmakelists).unwrap();
    assert_eq!(original_content, new_content, "CMakeLists.txt should be byte-for-byte unchanged");
}

#[test]
fn test_beru_run_inline_manifest_zero_context() {
    let p = project("zero_context")
        .file(
            "script_with_deps.cpp",
            r#"
            // /// beru
            // [package]
            // cxx-std = "c++20"
            // ///
            #include <iostream>
            int main() { std::cout << "Inline dep works!\n"; }
            "#,
        )
        .build();
    
    std::fs::remove_file(p.root().join("Beru.toml")).ok();

    p.beru("run")
        .arg("script_with_deps.cpp")
        .assert()
        .success()
        .stdout(predicate::str::contains("Inline dep works!"));
}

#[test]
fn test_beru_run_zero_context_no_deps() {
    let p = project("zero_context_no_deps")
        .file(
            "script_no_deps.cpp",
            r#"
            #include <iostream>
            int main() { std::cout << "Zero context no deps!\n"; }
            "#,
        )
        .build();
    
    std::fs::remove_file(p.root().join("Beru.toml")).ok();

    p.beru("run")
        .arg("script_no_deps.cpp")
        .assert()
        .success()
        .stdout(predicate::str::contains("Zero context no deps!"));
}

#[test]
fn test_beru_run_adhoc_cache_hit() {
    let p = project("adhoc_cache")
        .file(
            "script.cpp",
            r#"
            #include <iostream>
            int main() { std::cout << "Cache hit test!\n"; }
            "#,
        )
        .build();
    
    std::fs::remove_file(p.root().join("Beru.toml")).ok();

    p.beru("run")
        .arg("script.cpp")
        .assert()
        .success();

    p.beru("run")
        .arg("script.cpp")
        .assert()
        .success()
        .stdout(predicate::str::contains("cache hit"));
}

#[test]
fn test_beru_run_fallback_warns() {
    let p = project("fallback-warns")
        .file(
            "Beru.toml",
            r#"
            [package]
            name = "fallback-warns"
            version = "0.1.0"
            type = "executable"
            cxx-std = "c++17"
            "#,
        )
        .file(
            "script.cpp",
            r#"
            #include <iostream>
            int main() { std::cout << "Fallback running!\n"; }
            "#,
        )
        .build();

    p.beru("run")
        .arg("script.cpp")
        .assert()
        .success()
        .stderr(predicate::str::contains("running script using surrounding project's dependencies"));
}
