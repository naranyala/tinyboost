use std::sync::OnceLock;

/// A one-time initialization value that is computed on first access.
///
/// The initializer is called at most once; every subsequent access returns a
/// reference to the same value. `LazyInit` is `Sync` whenever `T` is, so it
/// can be shared across threads.
///
/// An initializer may be a plain function pointer or any closure that
/// implements `Fn() -> T` (closures may capture state by shared reference,
/// which makes call counting and configuration capture possible in tests).
///
/// # Examples
///
/// ```
/// use tinyboost::LazyInit;
///
/// let config = String::from("dev");
/// let lazy = LazyInit::new(|| format!("mode={config}"));
/// assert!(!lazy.is_initialized());
/// assert_eq!(lazy.get(), "mode=dev");
/// assert!(lazy.is_initialized());
/// ```
pub struct LazyInit<T, F = fn() -> T> {
    cell: OnceLock<T>,
    init: F,
}

impl<T, F> LazyInit<T, F>
where
    F: Fn() -> T,
{
    /// Creates a new lazy value with the given initializer.
    pub fn new(init: F) -> Self {
        Self {
            cell: OnceLock::new(),
            init,
        }
    }

    /// Returns a reference to the value, initializing it on first access.
    pub fn get(&self) -> &T {
        self.cell.get_or_init(|| (self.init)())
    }

    /// Returns `true` if the value has already been initialized.
    ///
    /// This never triggers initialization.
    pub fn is_initialized(&self) -> bool {
        self.cell.get().is_some()
    }

    /// Consumes the lazy value and returns the inner value if it was already
    /// initialized. Accessing this does not trigger initialization.
    pub fn into_inner(self) -> Option<T> {
        self.cell.into_inner()
    }
}
