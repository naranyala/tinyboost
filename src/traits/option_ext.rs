use crate::core::Maybe;

/// Option extension trait mirroring the [`ResultExt`](crate::ResultExt) hooks.
///
/// The same side-effect hook pattern is offered for option values, because
/// `Option` has no inherent `tap`-style methods.
pub trait OptionExt<T> {
    /// Runs a side-effect hook if the option is `Some`, then returns the
    /// option unchanged.
    fn tap_some(self, f: impl FnOnce(&T)) -> Self;

    /// Runs a side-effect hook if the option is `None`, then returns the
    /// option unchanged.
    fn tap_none(self, f: impl FnOnce()) -> Self;

    /// Converts the option into a `Maybe`, keeping the same value-or-empty
    /// semantics.
    fn into_maybe(self) -> Maybe<T>;
}

impl<T> OptionExt<T> for Option<T> {
    fn tap_some(self, f: impl FnOnce(&T)) -> Self {
        if let Some(ref value) = self {
            f(value);
        }
        self
    }

    fn tap_none(self, f: impl FnOnce()) -> Self {
        if self.is_none() {
            f();
        }
        self
    }

    fn into_maybe(self) -> Maybe<T> {
        Maybe::from(self)
    }
}
