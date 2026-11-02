//! Extension traits for `Result` and `Option`.

pub mod option_ext;
pub mod result_ext;

pub use self::option_ext::OptionExt;
pub use self::result_ext::ResultExt;
