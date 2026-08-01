#![warn(missing_docs)]

//! Beru recipe engine — fetch, verify, and prepare non-Beru-native packages.
//!
//! A recipe is a declarative `recipe.toml` describing how to download, verify,
//! and build a C++ package that doesn't ship its own `Beru.toml`. This is
//! analogous to vcpkg portfiles or Conan recipes, but expressed in TOML.

mod fetch;
mod recipe;
mod resolve;

pub use fetch::*;
pub use recipe::*;
pub use resolve::*;
