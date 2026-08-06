use crate::support::project;
use beru_core::cache::BeruCache;
use predicates::prelude::*;

#[test]
fn test_beru_cache_clean_removes_adhoc_only() {
    let p = project("cache_clean_adhoc")
        .file(
            "script.cpp",
            r#"
            #include <iostream>
            int main() { std::cout << "Test!\n"; }
            "#,
        )
        .build();

    let cache = BeruCache::with_root(p.beru_home().to_path_buf());
    std::fs::create_dir_all(cache.builds_dir()).unwrap();
    std::fs::create_dir_all(cache.sources_dir()).unwrap();

    p.beru("run").arg("script.cpp").assert().success();

    assert!(cache.adhoc_dir().read_dir().unwrap().count() > 0);

    p.beru("cache")
        .arg("clean")
        .arg("--adhoc")
        .assert()
        .success();

    assert!(cache.adhoc_dir().exists());
    assert_eq!(cache.adhoc_dir().read_dir().unwrap().count(), 0);
    assert!(cache.builds_dir().exists());
    assert!(cache.sources_dir().exists());
}

#[test]
fn test_beru_cache_clean_all_preserves_bin_and_recipes() {
    let p = project("cache_clean_all").build();
    let cache = BeruCache::with_root(p.beru_home().to_path_buf());
    cache.ensure_dirs().unwrap();

    let bin_marker = cache.bin_dir().join("marker.txt");
    let recipe_marker = cache.recipes_dir().join("marker.txt");
    std::fs::write(&bin_marker, "bin").unwrap();
    std::fs::write(&recipe_marker, "recipe").unwrap();

    std::fs::create_dir_all(cache.builds_dir().join("test")).unwrap();
    std::fs::create_dir_all(cache.sources_dir().join("test")).unwrap();
    std::fs::create_dir_all(cache.adhoc_dir().join("test")).unwrap();

    p.beru("cache").arg("clean").assert().success();

    assert!(bin_marker.exists());
    assert!(recipe_marker.exists());

    assert_eq!(cache.builds_dir().read_dir().unwrap().count(), 0);
    assert_eq!(cache.sources_dir().read_dir().unwrap().count(), 0);
    assert_eq!(cache.adhoc_dir().read_dir().unwrap().count(), 0);
}

#[test]
fn test_beru_cache_size_reports_nonzero_after_adhoc_run() {
    let p = project("cache_size")
        .file(
            "script.cpp",
            r#"
            #include <iostream>
            int main() { std::cout << "Test size!\n"; }
            "#,
        )
        .build();

    p.beru("run").arg("script.cpp").assert().success();

    let output = p.beru("cache").arg("size").output().unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();

    assert!(stdout.contains("adhoc"));
    assert!(!stdout.contains("adhoc      0 B"));
}

#[test]
fn test_beru_run_adhoc_prints_binary_path() {
    let p = project("adhoc_print_binary")
        .file(
            "script.cpp",
            r#"
            #include <iostream>
            int main() { std::cout << "Binary print test!\n"; }
            "#,
        )
        .build();

    p.beru("run")
        .arg("script.cpp")
        .assert()
        .success()
        .stdout(predicate::str::contains("binary:"));

    p.beru("run")
        .arg("script.cpp")
        .assert()
        .success()
        .stdout(predicate::str::contains("cache hit:"));
}
