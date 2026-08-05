#![warn(missing_docs)]

//! Beru manifest parser — `Beru.toml` schema and validation.
//!
//! This crate defines the data model for `Beru.toml` manifests and provides
//! parsing/validation logic. In Phase 1, dependencies are limited to
//! `git` (with tag/branch/rev) and `path` sources — no version ranges yet.

mod dependency;
mod error;
mod inline;
mod lockfile;
mod manifest;

pub use dependency::*;
pub use error::*;
pub use inline::*;
pub use lockfile::*;
pub use manifest::*;
