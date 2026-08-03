# Command Reference: `beru run`

The `beru run` command is a convenience utility designed to accelerate the edit-compile-test loop for application developers. It executes a full project build and, upon success, immediately spawns the resulting binary in the current terminal.

---

## 1. Usage Synopsis

```bash
beru run
```

## 2. Detailed Description

During active development of an executable, developers traditionally run `cmake --build build` followed immediately by `./build/my_app`. 

`beru run` collapses this workflow.

1.  **Implicit Build:** The command first invokes the exact same orchestration pipeline as `beru build`. It resolves dependencies, fetches tarballs, compiles the cache, synthesizes the toolchain, and invokes CMake.
2.  **Binary Discovery:** Upon a successful exit code from the underlying CMake invocation, Beru intelligently searches the `build/` output directory for the compiled artifact. It parses the `name` field from the `Beru.toml` manifest to locate the correct file (accounting for platform-specific extensions, like `.exe` on Windows).
3.  **Process Spawning:** Finally, Beru spawns the located binary as a child process, attaching it directly to your terminal's `stdout`, `stdin`, and `stderr` streams so you can interact with it normally.

### 2.1. Project Type Restrictions
This command is strictly reserved for projects defined with `type = "executable"` in their `Beru.toml`. 

If you attempt to invoke `beru run` within a project defined as `type = "library"` or `type = "header-only"`, the command will abort with an error. Libraries do not produce an executable entry point (`main()`) and therefore cannot be "run" in this manner. (If you wish to execute a library's test suite, you must currently run the compiled test binary manually from the `build/` directory).

---

## 3. Options and Flags

This command currently takes no options or flags. 

*(Future versions of Beru will introduce a `--` separator allowing you to pass arbitrary runtime arguments directly to the spawned child process, e.g., `beru run -- --config=prod.json`).*

---

## 4. Examples

**Building and executing a fresh executable project:**
```bash
beru new my_cli --type executable
cd my_cli
beru run
```
*Expected Output:*
```text
  Configuring my_cli v0.1.0
     Building my_cli
      Running `build/my_cli`
Hello from Beru!
```
