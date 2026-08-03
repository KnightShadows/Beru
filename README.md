<div align="center">
  <h1>📦 Beru</h1>
  <p><strong>A modern, fast, and declarative C++ package manager and build orchestrator, written in Rust.</strong></p>
  <a href="https://github.com/KnightShadows/Beru/actions"><img src="https://github.com/KnightShadows/Beru/workflows/CI/badge.svg" alt="Build Status"></a>
  <br/>
  <br/>
</div>

Beru brings the beloved **Cargo workflow** to the C++ ecosystem. It abstracts away the pain of CMake toolchains, finding dependencies, and resolving version conflicts, allowing you to focus purely on writing C++ code.

## 🌟 Features

- 📦 **Manifest-Driven**: Uses a simple, declarative `Beru.toml` manifest file for your project. No more sprawling `CMakeLists.txt`.
- 🚀 **PubGrub Version Solving**: Unlike vcpkg or Conan which often fail with cryptic errors on version conflicts, Beru uses the battle-tested [PubGrub algorithm](https://github.com/pubgrub-rs/pubgrub) to guarantee conflict-free version resolution and provide clear, human-readable explanations if dependencies are incompatible.
- 🏗️ **CMake Orchestration**: Automatically generates CMake toolchains and orchestrates the build process entirely behind the scenes, but seamlessly integrates with your existing legacy `CMakeLists.txt`.
- 🌐 **Global Index & Registry**: Fetches and builds third-party libraries using declarative `recipe.toml` files from the decentralized Beru registry.
- ⚡ **Global Binary Cache**: Dependencies are compiled *once* and cached globally on your machine.
- 🦀 **Powered by Rust**: Built for speed, safety, and reliability.

## 📖 Documentation

Everything you need to know about Beru is located in the comprehensive **[Beru Documentation](docs/Home.md)**. 

The manual covers:
- Getting Started (Installation & Creating Projects)
- Deep Dive into the `Beru.toml` Manifest
- Understanding the Global Package Registry
- How to write `recipe.toml` files for third-party libraries
- Full CLI Reference

## 🚀 Quick Start

### System Requirements
- **Git**
- **CMake** (>= 3.20)
- **A C++ Compiler** (GCC, Clang, or MSVC)

### Installation

**Linux & macOS**
```bash
curl -fsSL https://raw.githubusercontent.com/KnightShadows/Beru/main/install.sh | bash
```

**Windows** (Open PowerShell)
```powershell
irm https://raw.githubusercontent.com/KnightShadows/Beru/main/install.ps1 | iex
```

### Your First Project

```bash
beru new my_project
cd my_project
beru run
```
It's that easy.

## 🤝 Contributing

We welcome contributions! Please read our [Contributing Guidelines](CONTRIBUTING.md) to learn how to set up the development environment and submit pull requests.

## 📄 License

Beru is dual-licensed under the terms of both the **MIT License** and the **Apache License (Version 2.0)**. 
See the `LICENSE-MIT` and `LICENSE-APACHE` files for details.

Copyright (c) 2026 KnightShadows
