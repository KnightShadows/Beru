#![warn(missing_docs)]

//! Beru build orchestrator — CMake toolchain generation and invocation.
//!
//! This crate handles the build pipeline:
//! 1. Generate a `beru-toolchain.cmake` that wires resolved dependencies
//! 2. Invoke `cmake` to configure the project
//! 3. Invoke `cmake --build` to compile
//! 4. Install built artifacts into the global cache

mod adhoc;
mod cmake;
mod custom;
mod deps;
mod toolchain;

pub use adhoc::*;
pub use cmake::*;
pub use custom::*;
pub use deps::*;
pub use toolchain::*;
