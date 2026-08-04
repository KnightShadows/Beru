# Command Reference: `beru check`

The `beru check` command performs a fast, syntax-only compilation check over your C++ project, bypassing the linking phase entirely to provide rapid feedback.

---

## 1. Usage Synopsis

```bash
beru check
```

## 2. Detailed Description

When writing C++ code, developers often waste significant time waiting for CMake to link heavy executable binaries during rapid iterations. The linker is notoriously slow in C++ projects.

`beru check` solves this by orchestrating a specialized CMake build that compiles your source code with the `-fsyntax-only` flag (supported natively by GCC and Clang). This flag instructs the compiler to parse the code, perform semantic analysis, and emit any warnings or errors, but to halt before emitting object files (`.o` files) or invoking the linker.

Because `-fsyntax-only` breaks standard CMake compiler validation checks, `beru check` cleverly injects overrides into the CMake configuration to forcefully bypass the CMake linker tests (`CMAKE_CXX_COMPILER_WORKS`), allowing the rapid feedback loop to execute seamlessly.

---

## 3. Options and Flags

### `[TARGET]` (Optional Positional Argument)
If your `src/` directory contains multiple `.cpp` files, you can specify exactly which target to check.

```bash
beru check day1
```

### `--profile <PROFILE>`
**Default:** `debug`

Selects the build profile to use.

```bash
beru check --profile release
```

---

## 4. Examples

**Executing a fast syntax check:**
```bash
$ beru check
Checking check-proj v0.1.0 (c++17) [syntax-only]
  Finished check-proj checked successfully
```
