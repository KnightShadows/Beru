# Guide: Adding and Managing Dependencies

The ability to seamlessly pull in third-party code is the defining feature of any modern package manager. Beru supports three distinct mechanisms for declaring dependencies, each tailored to a specific phase of the software development lifecycle. 

All dependencies are declared within the `[dependencies]` table (or `[dev-dependencies]` for test-only code) in your `Beru.toml` manifest.

---

## 1. Registry Dependencies

The most common way to add a library is via the Beru global index. This is analogous to pulling a package from crates.io or npmjs.com. 

When a library is published to the Beru index, it is assigned a recipe that teaches Beru exactly how to compile it. You only need to provide the package name and a SemVer (Semantic Versioning) constraint.

```toml
[dependencies]
fmt = "11.0.2"
spdlog = "1.14.1"
boost-asio = "1.85.0"
```

### 1.1. How Registry Resolution Works

When you execute `beru build`, Beru performs the following under the hood:
1.  **Index Lookup:** It queries your local clone of the index (located at `~/.beru/index/`) for the directories named `fmt`, `spdlog`, and `boost-asio`.
2.  **Version Selection:** It invokes the PubGrub algorithm to select the highest version of each package that satisfies your constraints, as well as the constraints of all transitive dependencies.
3.  **Compilation & Caching:** It downloads the source tarballs specified in the recipes, compiles them, and stores the resulting binaries in the global cache (`~/.beru/cache/`). If a cached binary matching your compiler and C++ standard already exists, compilation is skipped entirely.

### 1.2. Updating the Index

Because the index is stored locally, it does not update itself automatically. If you know a new version of `fmt` was released yesterday but Beru says it cannot be found, your local index is stale.

Run the following command to synchronize your local index with the upstream GitHub repository:

```bash
beru index update
```

---

## 2. Git Dependencies

The global registry is excellent for stable, widely-used libraries. However, you will frequently need to depend on code that is not published to the registry. This occurs when:
*   You are relying on a proprietary, internal library hosted on your company's private GitHub/GitLab server.
*   You need a bleeding-edge bug fix from a specific commit on a public repository before an official release is cut.

For these scenarios, Beru allows you to depend directly on a Git repository.

### 2.1. The Git Syntax

To declare a Git dependency, provide a TOML inline table specifying the `git` URL and a pinning strategy.

```toml
[dependencies]
# Pin to a specific branch (e.g., 'main' or 'develop')
internal-auth = { git = "git@gitlab.company.com:backend/auth.git", branch = "main" }

# Pin to a specific Git tag
rapidjson = { git = "https://github.com/Tencent/rapidjson.git", tag = "v1.1.0" }

# Pin to a specific, immutable commit SHA
my-fork = { git = "https://github.com/myuser/library.git", rev = "9fceb02d0ae598e95dc970b709b4cb" }
```

### 2.2. Requirements for Git Dependencies

Unlike registry dependencies (which are powered by `recipe.toml` files hosted in the index), Beru must know how to build the code it clones from Git.

Therefore, **a Git repository must contain a valid `Beru.toml` file at its root** for Beru to consume it as a dependency. If the target repository uses raw CMake and does not have a `Beru.toml`, you cannot currently use it as a direct Git dependency; you must write a registry recipe for it instead.

### 2.3. The Lockfile Guarantee

If you pin to a `branch` (like `main`), the code on that branch changes over time. However, Beru guarantees deterministic builds. 

When you run `beru build` the first time, Beru clones the repository, resolves the `main` branch to a specific commit SHA (e.g., `a1b2c3d`), and writes that SHA into your `Beru.lock` file. 

Subsequent builds—or builds triggered by other developers who clone your repository—will read `Beru.lock` and checkout that exact `a1b2c3d` commit, ignoring any new commits pushed to `main` until you explicitly run `beru resolve` again.

---

## 3. Path Dependencies

During active development, you often need to modify an application and its dependencies simultaneously. Pushing a library to a Git server just to test a change in your application is unacceptably slow.

Path dependencies allow you to point Beru directly at a local directory on your filesystem.

```toml
[dependencies]
graphics-engine = { path = "../graphics-engine" }
```

### 3.1. Workflow Integration

Path dependencies are treated as first-class citizens in the dependency graph, but with a critical difference: **they are never globally cached.**

Because Beru assumes the code in a path dependency is under active development and changing rapidly, it is compiled directly into your local `.beru/` build context on every build. 

When you are ready to publish your application, you must replace the `path` dependencies with `git` or registry constraints, as path dependencies cannot be resolved by downstream consumers.

---

## 4. Development Dependencies

Often, you require libraries strictly for writing unit tests or benchmarks, such as GoogleTest or Catch2. You do not want these heavy testing frameworks to be compiled and linked into the final release binary, nor do you want them forced upon users who depend on your library.

The `[dev-dependencies]` section isolates these packages.

```toml
[dev-dependencies]
gtest = { git = "https://github.com/google/googletest.git", tag = "v1.15.0" }
```

When you run `beru build`, both `[dependencies]` and `[dev-dependencies]` are compiled. However, if another project adds your package to their `Beru.toml`, Beru will gracefully ignore your `[dev-dependencies]`, keeping the downstream dependency graph clean and compilation times fast.
