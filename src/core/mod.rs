//! Core utility types provided by tinyboost.
//!
//! This module contains the crate's standalone building blocks for cleanup,
//! lazy initialization, and optional values.

/// Scope-exit cleanup guard implementation and [`ScopeGuard`] type.
pub mod guard;
/// One-time lazy initialization implementation and [`LazyInit`] type.
pub mod lazy;
/// Explicit value-or-empty implementation and [`Maybe`] type.
pub mod maybe;

pub use guard::ScopeGuard;
pub use lazy::LazyInit;
pub use maybe::Maybe;
