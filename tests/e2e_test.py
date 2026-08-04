import os
import subprocess
import tempfile
import shutil
import sys

BERU_BIN = os.path.abspath("target/debug/beru")

if not os.path.exists(BERU_BIN):
    print(f"Error: Could not find beru binary at {BERU_BIN}. Please run 'cargo build' first.")
    sys.exit(1)

def run_cmd(cmd, cwd=None, expect_fail=False):
    print(f"Running: {' '.join(cmd)}")
    res = subprocess.run(cmd, cwd=cwd, capture_output=True, text=True)
    if not expect_fail and res.returncode != 0:
        print(f"Command failed with exit code {res.returncode}")
        print(f"STDOUT:\n{res.stdout}")
        print(f"STDERR:\n{res.stderr}")
        sys.exit(1)
    return res

def main():
    # Setup temporary directory
    temp_dir = tempfile.mkdtemp()
    print(f"Using temporary directory: {temp_dir}")
    
    try:
        # Test 1: beru new
        print("\n--- Test 1: beru new ---")
        run_cmd([BERU_BIN, "new", "test-proj", "--type", "executable"], cwd=temp_dir)
        proj_dir = os.path.join(temp_dir, "test-proj")
        assert os.path.exists(os.path.join(proj_dir, "Beru.toml")), "Beru.toml not created"
        assert os.path.exists(os.path.join(proj_dir, "CMakeLists.txt")), "CMakeLists.txt not created"
        assert os.path.exists(os.path.join(proj_dir, "src", "main.cpp")), "src/main.cpp not created"
        
        # Test 2: beru build
        print("\n--- Test 2: beru build ---")
        run_cmd([BERU_BIN, "build"], cwd=proj_dir)
        assert os.path.exists(os.path.join(proj_dir, "build", "test-proj")), "Binary not built"
        
        # Test 3: beru run
        print("\n--- Test 3: beru run ---")
        res = run_cmd([BERU_BIN, "run"], cwd=proj_dir)
        assert "Hello from Beru!" in res.stdout, "Unexpected run output"

        # Test 4: Ad-Hoc execution
        print("\n--- Test 4: beru run <script.cpp> ---")
        script_path = os.path.join(proj_dir, "src", "script.cpp")
        with open(script_path, "w") as f:
            f.write("#include <iostream>\nint main() { std::cout << \"Ad-Hoc execution works!\" << std::endl; return 0; }\n")
        
        res = run_cmd([BERU_BIN, "run", "src/script.cpp"], cwd=proj_dir)
        assert "Ad-Hoc execution works!" in res.stdout, "Ad-Hoc run output incorrect"
        
        cmakelists = open(os.path.join(proj_dir, "CMakeLists.txt")).read()
        assert "add_executable(script" in cmakelists, "Target not appended to CMakeLists.txt"
        assert "beru_link_dependencies(script)" in cmakelists, "Magic macro not appended to CMakeLists.txt"

        # Test 5: beru init
        print("\n--- Test 5: beru init ---")
        init_dir = os.path.join(temp_dir, "init-proj")
        os.makedirs(init_dir)
        run_cmd([BERU_BIN, "init", "--type", "library"], cwd=init_dir)
        assert os.path.exists(os.path.join(init_dir, "Beru.toml")), "Beru.toml not created in init"
        assert os.path.exists(os.path.join(init_dir, "src", "init-proj.cpp")), "src/init-proj.cpp not created"
        
        # Test 6: beru resolve
        print("\n--- Test 6: beru resolve ---")
        # Add a dependency to the executable project
        manifest_path = os.path.join(proj_dir, "Beru.toml")
        manifest_data = open(manifest_path).read()
        manifest_data = manifest_data.replace("[dependencies]", "[dependencies]\nfmt = \"11.0.2\"")
        with open(manifest_path, "w") as f:
            f.write(manifest_data)
            
        run_cmd([BERU_BIN, "resolve"], cwd=proj_dir)
        assert os.path.exists(os.path.join(proj_dir, "Beru.lock")), "Beru.lock not created"
        lock_contents = open(os.path.join(proj_dir, "Beru.lock")).read()
        assert "fmt" in lock_contents, "fmt not resolved in lockfile"
        
        print("\nAll E2E tests passed successfully!")
        
    finally:
        shutil.rmtree(temp_dir)

if __name__ == "__main__":
    main()
