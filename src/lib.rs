//! tinyboost is a small, explicit Rust utility crate inspired by the practical
//! breadth of C++ Boost without trying to become a giant collection of
//! everything.
//!
//! The crate is intentionally tiny: it focuses on a handful of foundational
//! utilities that many Rust codebases re-implement in ad hoc forms. The goal is
//! to provide clear, idiomatic building blocks without pulling in a large
//! dependency graph or hiding behavior behind clever abstractions.
#![deny(missing_docs)]

pub mod core;
pub mod prelude;
pub mod traits;

pub use crate::core::{LazyInit, Maybe, ScopeGuard};
pub use crate::traits::{OptionExt, ResultExt};

/// A tiny collection of general-purpose utilities for Rust codebases that want
/// a Boost-like layer of pragmatic building blocks without pulling in a huge
/// dependency graph.
pub mod tinyboost {}
