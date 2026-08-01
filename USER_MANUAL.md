# Beru User Manual

Welcome to the Beru user manual! Beru is designed to bring a Cargo-like experience to C++ development. This guide will walk you through everything you need to know to get started.

---

## 1. Getting Started

### Creating a New Project
To start a new C++ project, simply run:

```bash
beru new hello_world
cd hello_world
```

This will automatically create a new directory with the following structure:
```text
hello_world/
├── Beru.toml       # The project manifest
├── src/
│   └── main.cpp    # Your main C++ source file
```

### Initializing an Existing Project
If you already have a C++ project and want to start managing it with Beru, navigate to the folder and run:
```bash
beru init
```

---

## 2. Understanding `Beru.toml`

The `Beru.toml` file is the heart of your project. It uses a declarative syntax (TOML) to describe your project and its dependencies.

Here is a standard example:

```toml
[package]
name = "hello_world"
version = "0.1.0"
cxx-std = "c++20"     # Choose between c++11, c++14, c++17, c++20, c++23
type = "executable"   # 'executable', 'library', or 'header-only'
authors = ["Your Name <you@example.com>"]

[dependencies]
fmt = "11.0.2"        # Fetched from the global Beru index

[build]
system = "cmake"      # Beru orchestrates CMake automatically under the hood
```

### Dependency Types
You can specify dependencies in several ways:
- **Registry / Index**: `fmt = "11.0.2"`
- **Git Repository**: `json = { git = "https://github.com/nlohmann/json", tag = "v3.11.3" }`
- **Local Path**: `my_local_lib = { path = "../my_local_lib" }`

---

## 3. The Package Workflow

### Resolving Dependencies
Before building, Beru needs to figure out the exact versions of all dependencies to use. Run:
```bash
beru resolve
```
This uses the advanced **PubGrub algorithm** to resolve the dependency graph and guarantees there are no version conflicts. It then generates a `Beru.lock` file. You should commit `Beru.lock` to version control for executable projects to ensure reproducible builds for everyone.

### Building the Project
To compile your C++ project, run:
```bash
beru build
```
Behind the scenes, Beru will:
1. Fetch all third-party libraries (and their `recipe.toml` definitions).
2. Build the libraries automatically, caching the compiled binaries globally so they never have to be rebuilt.
3. Automatically generate a seamless CMake toolchain.
4. Compile your actual project into the `target/` directory.

### Running the Project
If your `Beru.toml` specifies `type = "executable"`, you can build and run your project in one command:
```bash
beru run
```
This will execute the resulting binary artifact located in `target/debug/hello_world`.

---

## 4. The Global Index

Beru avoids the need for heavy, centralized servers by using a **Decentralized Git Index** for its package registry. 

When you specify a package like `fmt = "11.0"`, Beru looks it up in your local copy of the index (stored in `~/.beru/index`). To ensure you have the latest available packages and versions, you should periodically update your index:

```bash
beru index update
```

This performs a fast git pull under the hood, instantly giving you access to the newest C++ packages!
