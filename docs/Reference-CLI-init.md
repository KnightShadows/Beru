# Command Reference: `beru init`

The `beru init` command is designed to bring an existing codebase into the Beru ecosystem safely and non-destructively. While `beru new` creates a project from scratch, `init` operates in-place on your current directory.

---

## 1. Usage Synopsis

```bash
beru init [OPTIONS]
```

## 2. Detailed Description

When executed, `beru init` intelligently analyzes the current working directory and applies the Beru orchestration model without overwriting your existing work.

1.  **Name Inference:** The command derives the package name from the name of the current directory, sanitizing it to ensure it contains only valid lowercase characters and hyphens.
2.  **Manifest Generation:** It drops a newly generated `Beru.toml` into the root directory. If a `Beru.toml` already exists, the command aborts gracefully with a warning, leaving the file untouched.
3.  **Non-Destructive Scaffolding:** It evaluates the state of your directory. If standard directories (`src/`, `tests/`) or files (`CMakeLists.txt`, `.gitignore`) are missing, it will generate them exactly as `beru new` would. **However, if any of these files or folders already exist, Beru will strictly ignore them.** It will never overwrite your existing source code or CMake configuration.

### 2.1. The "Legacy CMake" Workflow
`beru init` is the primary entry point for migrating legacy CMake projects. If you run `beru init` in a directory containing a massive, complex `CMakeLists.txt`, Beru will simply generate the `Beru.toml`. 

When you subsequently run `beru build`, Beru will orchestrate your dependencies, synthesize a toolchain, and then pass control *back* to your existing `CMakeLists.txt`, injecting the toolchain paths so your existing `find_package` calls succeed automatically.

---

## 3. Options and Flags

### `--type <TYPE>`
**Default:** `executable`

Dictates the `type` field written into the generated `Beru.toml`. If `init` determines it needs to scaffold missing source files, it will use this type to determine the boilerplate (e.g., generating `src/main.cpp` vs. `include/<name>.hpp`).

*   **`executable`**: For standalone applications.
*   **`library`**: For compiled libraries.
*   **`header-only`**: For template-heavy, uncompiled libraries.

### `--cxx-std <STD>`
**Default:** `c++17`

Specifies the C++ standard written into the `Beru.toml`. This standard will be enforced across all dependencies you subsequently add to the project.

*Valid options:* `c++11`, `c++14`, `c++17`, `c++20`, `c++23`, `c++26`.

---

## 4. Examples

**Initializing an existing legacy project as a C++14 library:**
```bash
cd legacy_physics_engine
beru init --type library --cxx-std c++14
```

**Quickly dropping a manifest into a scratch directory:**
```bash
mkdir test_sandbox && cd test_sandbox
beru init
```
