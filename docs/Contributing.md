# Contributing to the Beru Ecosystem

The Beru ecosystem is entirely open-source and driven by the C++ community. It thrives on developers taking the time to share their knowledge, fix bugs, and package their favorite libraries.

Because Beru abstracts away so much complexity, contributing is divided into two entirely separate domains: contributing to the **Global Package Index** (which requires zero knowledge of Rust), and contributing to the **Beru Core Orchestrator** (which requires a solid understanding of Rust and system programming).

This chapter outlines the expectations, workflows, and standards for both domains.

---

## 1. Contributing to the Global Index (Adding Packages)

If your favorite C++ library is not available when you run `beru build`, you do not need to wait for the Beru core team to add it. The index is decentralized, and anyone can submit a package.

### 1.1. The Scope of the Index
The index (hosted at `https://github.com/KnightShadows/beru_index`) is simply a Git repository containing `recipe.toml` files. It does not host the source code of the C++ libraries; it only hosts the instructions on how to build them.

Therefore, you do not need to be the author of a C++ library to publish it to the Beru index. You simply need to write a correct recipe for it.

### 1.2. The Contribution Workflow
1.  **Fork and Clone:** Fork the `beru_index` repository on GitHub and clone it locally.
2.  **Author the Recipe:** Create the appropriate `<package_name>/<version>/recipe.toml` directory structure. Follow the strict guidelines in the [Authoring a Recipe](Guide-Writing-A-Recipe.md) chapter.
3.  **Local Verification:** This is non-negotiable. You must test your recipe locally by building a dummy project against it. Submitting broken recipes wastes CI resources and reviewer time.
4.  **Submit a Pull Request:** Push to your fork and open a PR.

### 1.3. Review Standards for Recipes
The index maintainers are extremely strict regarding reproducibility and security. Your PR will be rejected if:
*   The `sha256` checksum is missing or incorrect.
*   The `url` points to a mutable Git branch (like `main`) instead of an immutable release tarball or specific `tag`.
*   The `cmake_targets` export the wrong target name, causing downstream linking failures.

---

## 2. Contributing to Beru Core (The CLI and Orchestrator)

The Beru orchestrator itself is written in Rust. It is a complex piece of systems software that interacts directly with the filesystem, external Git processes, and the CMake build system.

The core repository is located at `https://github.com/KnightShadows/beru`.

### 2.1. Setting Up Your Development Environment
To hack on the Beru core, you need a robust Rust toolchain.
1.  Install Rust via [rustup](https://rustup.rs/).
2.  Install CMake >= 3.20 (required for the integration test suite to successfully orchestrate dummy C++ projects).
3.  Clone the repository: `git clone https://github.com/KnightShadows/beru.git`

### 2.2. Understanding the Workspace Architecture
Beru is divided into several specialized crates to maintain separation of concerns. Before writing code, familiarize yourself with the domain you are modifying:

*   **`crates/beru-manifest/`**: Contains the Serde parsing logic for `Beru.toml`. If you are adding a new configuration field (like a new `[profile]` flag), start here.
*   **`crates/beru-resolve/`**: The brain of the operation. This crate integrates the PubGrub algorithm. It is highly mathematical and complex. Modify with extreme caution.
*   **`crates/beru-recipe/`**: Handles the parsing of `recipe.toml` files and the downloading/SHA-256 verification of source tarballs.
*   **`crates/beru-build/`**: The CMake orchestration engine. If you need to change how Beru invokes CMake or how it synthesizes the `beru-toolchain.cmake` file, this is the crate.
*   **`crates/beru-cli/`**: The `clap`-powered command-line interface. If you are adding a new subcommand (like `beru publish`), the entry point goes here.

### 2.3. The Pull Request Checklist
The Beru core team enforces strict CI checks. Before opening a PR, you must run the following locally and ensure they pass:

0.  **Git Hooks (Required):** You must configure your local git to use our strict pre-push hooks. This guarantees you never push broken code.
    ```bash
    git config core.hooksPath .githooks
    ```

1.  **Formatting:** We use standard `rustfmt`.
    ```bash
    cargo fmt --all
    ```
2.  **Linting (Clippy):** The codebase must compile with zero warnings. We enforce strict linting rules to prevent subtle logic bugs.
    ```bash
    cargo clippy --all-targets --all-features -- -D warnings
    ```
3.  **Unit and Integration Tests:** Your code must not break existing functionality. Furthermore, if you are adding a new feature, you *must* write a test for it.
    ```bash
    cargo test --workspace
    ```

If you have an idea for a massive architectural change (e.g., swapping CMake for Ninja directly, or replacing PubGrub), please open an Issue to discuss the design with the maintainers *before* spending weeks writing code!
