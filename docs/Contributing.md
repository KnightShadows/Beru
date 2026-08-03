# Contributing to Beru

We welcome contributions to both the **Beru tool** itself and the **Beru Index**!

## Contributing Recipes (The Index)

If you just want to add a new C++ library to Beru, you do **not** need to modify the Beru source code. 
Instead, you should contribute to the `beru_index` repository. Please read the [Publishing to the Registry](Guide-Publishing-To-The-Registry.md) guide for full instructions.

## Contributing to Core Beru

If you want to fix a bug or add a feature to the Beru CLI, follow these steps:

### 1. Dev Environment
Beru is written in Rust. You will need:
- The standard Rust toolchain (via `rustup`).
- Cargo (included with `rustup`).
- CMake (for testing the C++ orchestration).

### 2. Architecture Map
Familiarize yourself with the crate layout:
- `crates/beru-manifest/`: Parsing `Beru.toml`.
- `crates/beru-resolve/`: PubGrub solver implementation.
- `crates/beru-recipe/`: Fetching and parsing `recipe.toml` files.
- `crates/beru-build/`: The core engine that orchestrates CMake.
- `crates/beru-cli/`: The CLI entry point and Clap configuration.

### 3. PR Expectations
- **Format your code**: Run `cargo fmt` before submitting.
- **Pass Clippy**: Run `cargo clippy --all-targets` and ensure there are no warnings.
- **Tests**: Write unit tests for your changes. Run `cargo test` to ensure existing tests pass.

