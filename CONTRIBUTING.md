# Contributing to Beru

Thank you for your interest in contributing to Beru! 

We welcome contributions of all kinds: bug fixes, new features, improvements to the recipe engine, and documentation updates.

## Architecture Overview

Beru is written in Rust and is split into several interconnected crates in the `crates/` directory:

- **`beru-cli`**: The main executable and command-line parser (`beru build`, `beru run`, etc.).
- **`beru-core`**: Core utilities, cache management, ABI hashing, and compiler toolchain detection.
- **`beru-manifest`**: Parsing and validating the `Beru.toml` and `Beru.lock` schemas.
- **`beru-recipe`**: The recipe engine that fetches and processes third-party C++ dependencies using declarative `recipe.toml` files.
- **`beru-resolve`**: Bridging Beru's dependency graphs with the `pubgrub` version-solving algorithm.
- **`beru-build`**: Orchestrating CMake execution and invoking system compilers.

## Development Setup

1. **Prerequisites**: Ensure you have [Rust](https://rustup.rs/) (v1.85+) and a working C++ compiler (GCC, Clang, or MSVC) installed.
2. **Clone the repository**:
   ```bash
   git clone https://github.com/KnightShadows/Beru.git
   cd Beru
   ```
3. **Build the project**:
   ```bash
   cargo build --workspace
   ```
4. **Run the tests**:
   ```bash
   cargo test --workspace
   ```

## Pull Request Guidelines

Before submitting a Pull Request, please ensure the following:

1. **Formatting**: Run `cargo fmt --all` to ensure your code matches the project's formatting standard.
2. **Linting**: We enforce strict linting. Run `cargo clippy --workspace --all-targets -- -D warnings` and ensure there are no warnings.
3. **Documentation**: All public items must be documented! The project enforces `#![warn(missing_docs)]`. If you add a new public struct, enum, function, or module, please provide a comprehensive `///` rustdoc comment.
4. **Testing**: If you are adding a new feature, please try to add a corresponding unit test or integration test.

We use **GitHub Actions** for our CI pipeline. Your PR will automatically be tested against Linux, macOS, and Windows.

Thank you for helping make C++ package management better!
