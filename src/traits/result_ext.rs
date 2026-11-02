/// Result utilities for explicit side-effect hooks without altering the value.
pub trait ResultExt<T, E> {
    fn tap_ok(self, f: impl FnOnce(&T)) -> Self;
    fn tap_err(self, f: impl FnOnce(&E)) -> Self;
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
}
