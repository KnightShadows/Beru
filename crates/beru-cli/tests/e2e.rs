use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;
use tempfile::tempdir;

#[test]
fn test_beru_e2e_workflow() -> Result<(), Box<dyn std::error::Error>> {
    // 1. Create a fresh temporary directory
    let dir = tempdir()?;
    let temp_path = dir.path();

    // 2. Test: beru new
    let mut cmd = Command::cargo_bin("beru")?;
    cmd.env("BERU_HOME", temp_path.join(".beru"))
        .arg("new")
        .arg("test-proj")
        .arg("--type")
        .arg("executable")
        .current_dir(temp_path)
        .assert()
        .success()
        .stdout(predicate::str::contains("Created project `test-proj`"));

    let proj_dir = temp_path.join("test-proj");
    assert!(proj_dir.join("Beru.toml").exists());
    assert!(proj_dir.join("CMakeLists.txt").exists());
    assert!(proj_dir.join("src").join("main.cpp").exists());

    // 3. Test: beru build
    let mut cmd = Command::cargo_bin("beru")?;
    cmd.env("BERU_HOME", temp_path.join(".beru"))
        .arg("build")
        .current_dir(&proj_dir)
        .assert()
        .success()
        .stdout(predicate::str::contains("built successfully"));

    // 4. Test: beru run
    let mut cmd = Command::cargo_bin("beru")?;
    cmd.env("BERU_HOME", temp_path.join(".beru"))
        .arg("run")
        .current_dir(&proj_dir)
        .assert()
        .success()
        .stdout(predicate::str::contains("Hello from Beru!"));

    // 5. Test: beru run <ad-hoc script>
    let script_path = proj_dir.join("src").join("script.cpp");
    fs::write(
        &script_path,
        "#include <iostream>\nint main() { std::cout << \"Ad-Hoc works!\" << std::endl; return 0; }\n",
    )?;

    let mut cmd = Command::cargo_bin("beru")?;
    cmd.env("BERU_HOME", temp_path.join(".beru"))
        .arg("run")
        .arg("src/script.cpp")
        .current_dir(&proj_dir)
        .assert()
        .success()
        .stdout(predicate::str::contains("Ad-Hoc works!"));

    let cmakelists = fs::read_to_string(proj_dir.join("CMakeLists.txt"))?;
    assert!(cmakelists.contains("add_executable(script"));
    assert!(cmakelists.contains("beru_link_dependencies(script)"));

    // 6. Test: beru init
    let init_dir = temp_path.join("init-proj");
    fs::create_dir(&init_dir)?;

    let mut cmd = Command::cargo_bin("beru")?;
    cmd.env("BERU_HOME", temp_path.join(".beru"))
        .arg("init")
        .arg("--type")
        .arg("library")
        .current_dir(&init_dir)
        .assert()
        .success()
        .stdout(predicate::str::contains("Initializing Beru"));

    assert!(init_dir.join("Beru.toml").exists());
    assert!(init_dir.join("src").join("init-proj.cpp").exists());

    // 7. Test: beru resolve
    let manifest_path = proj_dir.join("Beru.toml");
    let manifest = fs::read_to_string(&manifest_path)?;
    let new_manifest = manifest.replace("[dependencies]", "[dependencies]\nfmt = \"11.0.2\"");
    fs::write(&manifest_path, new_manifest)?;

    // First update the index
    let mut cmd = Command::cargo_bin("beru")?;
    cmd.env("BERU_HOME", temp_path.join(".beru"))
        .arg("index")
        .arg("update")
        .assert()
        .success();

    let mut cmd = Command::cargo_bin("beru")?;
    cmd.env("BERU_HOME", temp_path.join(".beru"))
        .arg("resolve")
        .current_dir(&proj_dir)
        .assert()
        .success();

    let lockfile = fs::read_to_string(proj_dir.join("Beru.lock"))?;
    assert!(lockfile.contains("fmt"));

    Ok(())
}
