pub mod core;
pub mod prelude;
pub mod traits;

pub use crate::core::{LazyInit, Maybe, ScopeGuard};
pub use crate::traits::ResultExt;

/// A tiny collection of general-purpose utilities for Rust codebases that want
/// a Boost-like layer of pragmatic building blocks without pulling in a huge
/// dependency graph.
pub mod tinyboost {}
