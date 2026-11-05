//! Extension traits for `Result` and `Option` values.

/// Methods for observing and mapping `Result` values without changing control flow.
pub mod result_ext;
/// Methods for observing `Option` values and converting them to [`Maybe`](crate::Maybe).
pub mod option_ext;

pub use self::option_ext::OptionExt;
pub use self::result_ext::ResultExt;
