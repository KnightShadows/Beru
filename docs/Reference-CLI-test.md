# Command Reference: `beru test`

The `beru test` command compiles your project and immediately executes your project's test suite, automatically running tests in parallel.

---

## 1. Usage Synopsis

```bash
beru test
```

## 2. Detailed Description

`beru test` is a convenience wrapper around CMake's native testing tool, CTest. When invoked, it performs the following orchestration:

1. **Build Phase:** It implicitly invokes `beru build` (using the same internal compilation logic) to ensure all object files and test executables are up to date.
2. **Test Phase:** It spawns `ctest` inside the project's generated `build/` directory.

### 2.1. Automatic Parallelism
Modern C++ testing frameworks (like GTest, Catch2, or doctest) can take significant time to run. Beru automatically detects the number of available physical and logical CPU cores on your host machine and invokes `ctest -j <cores>`. This ensures your test suite runs as fast as mathematically possible without requiring you to pass manual parallelization flags.

### 2.2. Failure Outputs
Beru configures CTest to run with `--output-on-failure`. This means if a test passes, it remains silent (keeping your CI logs clean), but if it fails, it immediately dumps the standard output and standard error of the failing test directly to your terminal.

---

## 3. Options and Flags

### `--profile <PROFILE>`
**Default:** `debug`

Selects the build profile to use when compiling the test executables before running them.

```bash
beru test --profile release
```

---

## 4. Examples

**Executing the test suite:**
```bash
$ beru test
Building test-proj v0.1.0 (c++17)
  Finished test-proj built successfully
Running tests...
100% tests passed, 0 tests failed out of 5
```
