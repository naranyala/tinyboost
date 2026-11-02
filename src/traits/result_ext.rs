/// Result utilities for explicit side-effect hooks and mapping helpers
/// without changing control flow.
pub trait ResultExt<T, E> {
    /// Runs a side-effect hook if the result is `Ok`, then returns the result
    /// unchanged.
    fn tap_ok(self, f: impl FnOnce(&T)) -> Self;

    /// Runs a side-effect hook if the result is `Err`, then returns the result
    /// unchanged.
    fn tap_err(self, f: impl FnOnce(&E)) -> Self;

    /// Maps the success value to a new success type while keeping the error
    /// type and value.
    ///
    /// Named `map_ok` to stay distinct from [`Result::map`] while reading the
    /// same way in chains.
    fn map_ok<U, F>(self, f: F) -> Result<U, E>
    where
        F: FnOnce(T) -> U;
}

impl<T, E> ResultExt<T, E> for Result<T, E> {
    fn tap_ok(self, f: impl FnOnce(&T)) -> Self {
        if let Ok(ref value) = self {
            f(value);
        }
        self
    }

    fn tap_err(self, f: impl FnOnce(&E)) -> Self {
        if let Err(ref value) = self {
            f(value);
        }
        self
    }

    fn map_ok<U, F>(self, f: F) -> Result<U, E>
    where
        F: FnOnce(T) -> U,
    {
        match self {
            Ok(value) => Ok(f(value)),
            Err(error) => Err(error),
        }
    }
}
