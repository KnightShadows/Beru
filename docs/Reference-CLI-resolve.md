# Command Reference: `beru resolve`

The `beru resolve` command isolates the dependency resolution phase of the orchestration pipeline. It computes the exact dependency graph and generates the `Beru.lock` file without executing any downloads, CMake configurations, or compiler invocations.

---

## 1. Usage Synopsis

```bash
beru resolve
```

## 2. Detailed Description

Dependency resolution in C++ is a mathematically complex operation governed by strict constraints (the One Definition Rule). When you declare multiple dependencies in your `Beru.toml` (which in turn have their own transitive dependencies), you create a massive web of version constraints.

When you execute `beru resolve`, the CLI hands this web to the **PubGrub algorithm**—a state-of-the-art version solver.

### 2.1. The Execution Flow
1.  **Manifest Parsing:** The command reads your `Beru.toml`.
2.  **Constraint Evaluation:** It queries the local Git index (`~/.beru/index/`) to find available versions of your direct dependencies. It reads their respective `recipe.toml` files to gather their transitive dependencies.
3.  **Conflict Resolution:** PubGrub attempts to find a single, unified graph of package versions where no two packages demand mutually exclusive versions of a shared transitive dependency.
4.  **Locking:** If a valid solution exists, the exact versions (and their cryptographic checksums) are written to `Beru.lock`.

### 2.2. Error Handling
If the constraints are mathematically impossible to satisfy (e.g., Package A strictly requires `fmt = 10.0.0` and Package B strictly requires `fmt = 11.0.0`), the command will fail. 

Crucially, it will output a deeply detailed, human-readable trace of the exact conflict chain, explaining exactly why the resolution failed so you can adjust your `Beru.toml` constraints accordingly.

### 2.3. Idempotency
`beru resolve` is an idempotent operation relative to the state of the index. If a valid `Beru.lock` already exists, and your `Beru.toml` has not been modified since the lockfile was generated, the command will exit immediately without performing any calculations.

To force a re-evaluation of the graph (e.g., to pull in a newer version of a library that satisfies an existing constraint), you must manually delete the `Beru.lock` file or run `beru index update` before resolving.

---

## 3. Options and Flags

This command currently takes no options or flags. 

*(Future versions of Beru may introduce flags like `--update <package>` to selectively bump the locked version of a specific dependency).*

---

## 4. Examples

**Manually generating the lockfile before committing to version control:**
```bash
beru resolve
git add Beru.toml Beru.lock
git commit -m "chore: lock dependencies"
```
