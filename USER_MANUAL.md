# The Beru Book: Comprehensive User Manual

Welcome to **The Beru Book**! This is the definitive guide to using Beru, the modern C++ package manager and build orchestrator. Beru is designed to bring a Cargo-like experience to C++ development, abstracting away the pain of CMake toolchains, dependency resolution, and caching.

---

## Table of Contents
1. [Introduction](#1-introduction)
2. [Getting Started](#2-getting-started)
3. [The Manifest (`Beru.toml`)](#3-the-manifest-berutoml)
4. [The Registry & Index](#4-the-registry--index)
5. [Recipes & Third-Party Code](#5-recipes--third-party-code)
6. [Architecture & Build Engine](#6-architecture--build-engine)
7. [CLI Reference](#7-cli-reference)

---

## 1. Introduction

C++ has historically suffered from fragmented build systems and difficult dependency management. While tools like CMake, vcpkg, and Conan exist, they often require writing complex scripts or manually managing toolchains.

**Beru** changes this by introducing:
- **Declarative Manifests:** You only ever write a simple `Beru.toml` file. No CMake scripting required for your project.
- **PubGrub Resolution:** Beru uses the battle-tested PubGrub algorithm (used by modern package managers like Dart's `pub` and Python's `uv`) to guarantee conflict-free version resolution.
- **Decentralized Registry:** Packages are indexed via a decentralized Git repository instead of a heavy central server.
- **Global Binary Caching:** Dependencies are compiled once per system and cached globally. You never compile `fmt` or `boost` twice.

---

## 2. Getting Started

### Installation

If you haven't installed Beru yet, the easiest way is via our installer scripts:

**Linux & macOS:**
```bash
curl -fsSL https://raw.githubusercontent.com/KnightShadows/Beru/main/install.sh | bash
```

**Windows (PowerShell):**
```powershell
irm https://raw.githubusercontent.com/KnightShadows/Beru/main/install.ps1 | iex
```

### Creating a New Project

To start a new C++ project from scratch, simply run:
```bash
beru new my_app
cd my_app
```

This creates a new folder with the following structure:
```text
my_app/
├── Beru.toml       # The project manifest
├── src/
│   └── main.cpp    # Your main C++ source file
```

### Initializing an Existing Project

If you already have a folder with C++ source files and want to start managing it with Beru, navigate to the folder and run:
```bash
beru init
```
This will generate a `Beru.toml` file in the current directory.

### Running the Project

If your project is an executable, you can build and run it in a single command:
```bash
beru run
```
Beru will automatically resolve dependencies, generate a CMake toolchain, compile your project into the `target/` directory, and execute the resulting binary.

---

## 3. The Manifest (`Beru.toml`)

The `Beru.toml` file is the heart of your project. It uses TOML (Tom's Obvious, Minimal Language) syntax.

### The `[package]` Section

This section defines the metadata and core properties of your project.

```toml
[package]
name = "my_app"
version = "0.1.0"
cxx-std = "c++20"     # Options: c++11, c++14, c++17, c++20, c++23
type = "executable"   # Options: 'executable', 'library', 'header-only'
authors = ["Your Name <you@example.com>"]
```

- **`cxx-std`**: The C++ standard to use for compilation. Beru propagates this requirement to all dependencies.
- **`type`**: 
  - `executable`: Compiles to a standalone binary.
  - `library`: Compiles to a static/shared library for other projects to consume.
  - `header-only`: A library that only contains header files (no compilation required).

### The `[dependencies]` Section

This section lists the third-party libraries your project requires.

```toml
[dependencies]
# 1. Registry Dependency (Fetched from global index)
fmt = "11.0.2"

# 2. Git Dependency (Fetched directly from a Git repository)
json = { git = "https://github.com/nlohmann/json", tag = "v3.11.3" }

# 3. Local Path Dependency (References another Beru project locally)
my_local_lib = { path = "../my_local_lib" }
```

### The `[build]` Section

Defines how the project itself should be built. Currently, Beru orchestrates CMake under the hood for your project.

```toml
[build]
system = "cmake"
```

---

## 4. The Registry & Index

Beru avoids the need for a massive, centralized server infrastructure by using a **Decentralized Git Index** for its package registry.

When you specify a dependency like `fmt = "11.0.2"`, Beru doesn't query an API. Instead, it looks up the package in your local copy of the Beru Index.

### Updating the Index

To ensure you have access to the latest packages and versions, you should periodically update your local index:
```bash
beru index update
```
This performs a fast `git pull` on the index repository stored in `~/.beru/index`.

### The Lockfile (`Beru.lock`)

When you run `beru resolve` or `beru build`, Beru reads your `Beru.toml` and calculates the exact dependency graph using the **PubGrub algorithm**. The exact versions chosen are written to `Beru.lock`.

> [!TIP]
> If you are building an **executable**, you should commit `Beru.lock` to version control. This ensures that everyone building your project gets the exact same dependency versions. If you are building a **library**, you should add `Beru.lock` to your `.gitignore`.

---

## 5. Recipes & Third-Party Code

The Beru Index doesn't store source code or binaries. Instead, it stores **Recipes**.

A Recipe is a `recipe.toml` file that tells Beru how to fetch, build, and link a specific third-party library. If you want to contribute a new library to the global Beru ecosystem, you write a Recipe for it.

### Example: `recipe.toml` for `fmt`

```toml
[package]
name = "fmt"
version = "11.0.2"

[source]
# Where to download the source code
git = "https://github.com/fmtlib/fmt.git"
tag = "11.0.2"

[build]
# How to compile the library
system = "cmake"
args = ["-DFMT_TEST=OFF", "-DFMT_DOC=OFF"]

[export]
# What this library provides to downstream consumers
include_dirs = ["include"]
libs = ["fmt"]
```

When Beru encounters a dependency, it reads the Recipe, downloads the source, builds the library, and caches the result. The `[export]` block tells Beru exactly which include directories and library files to pass to your project's compiler.

---

## 6. Architecture & Build Engine

Beru operates through a sophisticated, multi-stage orchestration engine (primarily driven by the `beru-build` crate):

1. **Resolution Phase**: The `beru-manifest` and `beru-resolve` crates parse your manifest and use PubGrub to compute the dependency graph.
2. **Fetch & Build Phase**: `beru-recipe` downloads the source code for all third-party libraries. If a compiled binary doesn't already exist in your global cache (`~/.beru/cache`), Beru invokes the local build system (e.g., CMake) to compile the library *in isolation*.
3. **Orchestration Phase**: `beru-build` gathers all the `[export]` data from the compiled dependencies (include paths, static libraries, shared objects). It dynamically generates a `CMakeLists.txt` (and a custom CMake toolchain file) for *your* project, seamlessly injecting all the dependency flags.
4. **Compilation Phase**: Finally, Beru invokes CMake on your project inside the `target/` directory. 

Because dependencies are built in isolation and cached by a hash of their version and build flags, changing your project's source code never causes a rebuild of your dependencies.

---

## 7. CLI Reference

Beru provides a familiar, intuitive command-line interface.

- **`beru new <name>`**: Creates a new C++ project directory with a default manifest and `src/main.cpp`.
- **`beru init`**: Initializes an existing directory as a Beru project.
- **`beru resolve`**: Computes the dependency graph and generates/updates the `Beru.lock` file.
- **`beru build`**: Resolves dependencies, fetches sources, builds missing dependencies, and compiles the project into the `target/` directory.
- **`beru run`**: Executes `beru build` and immediately runs the resulting executable artifact.
- **`beru index update`**: Pulls the latest package definitions from the decentralized registry.
- **`beru help`**: Displays the full list of commands and options.

---
*Happy Coding with Beru!*
