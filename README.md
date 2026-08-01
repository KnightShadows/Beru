# Beru

**Beru** is a modern, fast, and secure C++ package manager and build orchestrator, written in Rust. It takes inspiration from Cargo, giving C++ developers a reliable and familiar workflow to manage dependencies, construct build graphs, and compile projects seamlessly.

## Features

- 📦 **Manifest-Driven**: Uses a simple, declarative `Beru.toml` manifest file for your project (similar to `Cargo.toml`).
- 🚀 **PubGrub Version Solving**: Utilizes the battle-tested PubGrub algorithm to perfectly resolve your dependency graph without version conflicts.
- 🏗️ **CMake Orchestration**: Automatically generates CMake toolchains and orchestrates the build process behind the scenes.
- 🌐 **Global Index & Recipe Engine**: Fetches and builds third-party libraries using declarative `recipe.toml` files from the global Beru registry, automatically caching binaries.
- 🦀 **Powered by Rust**: Built for speed and safety.

## Installation

### Linux & macOS 
The easiest way to install Beru on Linux or macOS is using our installer script. This will download the latest pre-compiled binary for your system:

```bash
curl -fsSL https://raw.githubusercontent.com/KnightShadows/Beru/main/install.sh | bash
```


### Windows
The easiest way to install Beru on Windows is using our PowerShell installer script. Open PowerShell and run:

```powershell
irm https://raw.githubusercontent.com/KnightShadows/Beru/main/install.ps1 | iex
```

### Cargo (All Platforms)
If you already have [Rust](https://rustup.rs/) installed, you can easily install Beru directly from crates.io:

```bash
cargo install beru
```

### Build from Source (All Platforms)
If you prefer to build the latest development version from source, you can clone the repository:

```bash
git clone https://github.com/KnightShadows/Beru.git
cd Beru
cargo install --path crates/beru-cli
```

## Usage

Beru provides a familiar, Cargo-like command-line interface for managing your C++ projects.

> [!TIP]
> **New to Beru?** Check out the comprehensive **[User Manual](USER_MANUAL.md)** for a full tutorial on creating projects, understanding the `Beru.toml` manifest, managing dependencies, and building your C++ code!

### Quick Reference

```bash
beru new my_project    # Create a new C++ project
beru resolve           # Resolve dependencies and generate Beru.lock
beru build             # Fetch dependencies and compile the project
beru run               # Build and execute the resulting binary
beru index update      # Update the global package registry
```

## License

Beru is dual-licensed under the terms of both the **MIT License** and the **Apache License (Version 2.0)**. 
See the `LICENSE-MIT` and `LICENSE-APACHE` files for details.

Copyright (c) 2026 KnightShadows
