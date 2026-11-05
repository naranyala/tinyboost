//! Curated re-exports for the most commonly used tinyboost utilities.
//!
//! Importing this module brings the core types and extension traits into scope.
pub use crate::core::{LazyInit, Maybe, ScopeGuard};
pub use crate::traits::{OptionExt, ResultExt};
