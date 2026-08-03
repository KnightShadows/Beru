# Beru: The Definitive Guide

Welcome to the definitive manual for **Beru**, the modern package manager and build orchestrator for C++. This book is designed to take you from a complete novice to an advanced practitioner capable of managing complex, cross-platform C++ dependency graphs.

Whether you are building a small command-line utility, a sprawling enterprise application, or a high-performance graphics engine, this guide will teach you how to leverage Beru to eliminate build system friction and focus on what matters most: writing excellent C++ code.

---

## 1. The C++ Packaging Problem

To understand Beru, we must first understand the problem it solves. Historically, C++ has lacked a unified, standardized approach to dependency management. When a C++ developer wants to use a third-party library (for example, the popular `{fmt}` formatting library or `nlohmann_json`), they typically face a daunting set of choices:

1. **System Package Managers (apt, brew, pacman):** Fast and easy, but severely limit reproducibility. If Developer A is on Ubuntu 20.04 and Developer B is on macOS, they will almost certainly receive different versions of the library, leading to divergent behavior and "it works on my machine" bugs.
2. **Git Submodules:** Pulling the source code directly into your repository ensures version consistency. However, it forces you to integrate the library's build system (usually CMake) into your own, which often requires complex incantations of `add_subdirectory()` and dealing with conflicting compiler flags.
3. **Dedicated Package Managers (vcpkg, Conan):** These represent a massive leap forward. However, they traditionally require the user to bridge the gap between the package manager and the build system manually. You must still write extensive CMake boilerplate, define custom toolchain files, and manage targets.

## 2. The Beru Philosophy

Beru was designed from the ground up to bring the **Cargo workflow** (from the Rust ecosystem) to C++. It operates on a set of core philosophical principles:

### 2.1. Declarative Manifests over Imperative Scripts
With Beru, you do not write build scripts; you write *manifests*. You declare what your project is, what C++ standard it requires, and what dependencies it needs in a clean, human-readable `Beru.toml` file. Beru infers the rest.

### 2.2. Deterministic, Mathematical Dependency Resolution
Dependency hell occurs when Package A needs `Library X v1` and Package B needs `Library X v2`. Beru uses the **PubGrub algorithm**—a mathematically rigorous, state-of-the-art version solver. If a conflict exists, Beru will not guess; it will provide a precise, human-readable trace of exactly why the graph cannot be resolved, allowing you to fix it immediately.

### 2.3. Zero Boilerplate Orchestration
Beru is not just a package manager; it is a **build orchestrator**. When you type `beru build`, Beru does not just download libraries and stop. It dynamically synthesizes the underlying CMake toolchains required to build those libraries, compiles them, and injects the resulting static or shared objects directly into your project's build context. You never have to write a `CMakeLists.txt` file unless you want to.

### 2.4. Decentralization and Autonomy
Unlike centralized ecosystems that rely on a single point of failure (like npmjs.com or PyPI), Beru's package index is entirely decentralized. The global registry is simply a Git repository containing lightweight instructions (called *recipes*) on how to build third-party code. You can easily host a private, internal Git registry for your company and use it seamlessly alongside the public index.

## 3. How to Use This Book

This manual is structured into several distinct sections, following the Diátaxis framework to suit your immediate needs:

*   **[🚀 Start Here](Getting-Started.md)**: If you are new to Beru, start with the Getting Started guide. It will walk you through installation and scaffolding your very first project. If you are coming from existing tools, read the [Migration Guide](Migrating-From-CMake-vcpkg-Conan.md).
*   **[📖 Guides](Guide-Adding-A-Dependency.md)**: These are task-oriented tutorials. Read these when you want to achieve a specific goal, such as adding a dependency from a private Git repository or writing a recipe to publish a new library to the world.
*   **[📚 Reference](Reference-Manifest-BeruToml.md)**: The exhaustive, technical specification of every configuration file and command-line flag. Use this section when you need to know exactly what `shared-libs = true` does or what flags `beru resolve` accepts.
*   **[🧠 Explanation](Concepts-Dependency-Resolution.md)**: Deep dives into the internal mechanics of Beru. Read these to understand the computer science behind the PubGrub algorithm or the architectural pipeline that turns TOML into compiled assembly.
*   **[🛠️ Community](Contributing.md)**: Guidelines on how to contribute to the Beru ecosystem, and a comprehensive FAQ for troubleshooting errors.

By mastering the concepts in this book, you will dramatically accelerate your C++ development cycle, ensuring that your projects compile flawlessly on any machine, every single time.
