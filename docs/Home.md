# Welcome to Beru

**Beru** is an open-source C++ package manager and build orchestrator written in Rust. It brings a Cargo-like, declarative workflow to C++ development, eliminating the boilerplate of hand-written CMake scripts and the friction of manually resolving C++ dependencies.

## Why Beru?

Historically, C++ development has been plagued by missing dependencies, complex CMake integrations, and the notorious "it works on my machine" problem. Existing tools like `vcpkg` and `Conan` help distribute packages, but they often require deep knowledge of CMake toolchains, custom triplets, or significant boilerplate to integrate into a project.

Beru takes a different approach: **it orchestrates the entire build process.**

You declare your project metadata and dependencies in a simple `Beru.toml` file, and Beru handles the rest: fetching, resolving, configuring, and building.

## Core Features

- 📦 **Declarative Manifest (`Beru.toml`)**: Define your project, C++ standard, and dependencies in a clean, TOML-based format.
- 🧩 **PubGrub Resolution**: Conflict-free, automated version solving powered by the same algorithm used by Cargo and npm.
- 🌍 **Decentralized Registry**: The package index is a lightweight Git repository. No central server downtime, and you can easily host your own private indexes.
- ⚡ **Global Binary Cache**: Dependencies are compiled exactly once per platform/architecture and instantly reused across all your local projects.
- 🛠️ **Zero CMake Boilerplate**: Beru automatically generates the underlying CMake toolchains and orchestrates the build behind the scenes. You don't have to write a single line of `CMakeLists.txt` unless you want to.

## Where to go next?

- **[Getting Started](Getting-Started.md)**: Install Beru and create your first project.
- **[Migrating from CMake/vcpkg/Conan](Migrating-From-CMake-vcpkg-Conan.md)**: See a side-by-side comparison of how Beru simplifies your workflow.

