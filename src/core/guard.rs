//! RAII-style cleanup that runs when a scope guard is dropped.

/// A scope guard that runs a closure when the guard is dropped.
///
/// This is a lightweight RAII pattern that helps ensure cleanup or finalization
/// occurs in a single, obvious place. The closure runs during panic unwinding as
/// well as on ordinary scope exit. Call [`dismiss`](Self::dismiss) to cancel it.
pub struct ScopeGuard<F>
where
    F: FnOnce(),
{
    closure: Option<F>,
}

impl<F> ScopeGuard<F>
where
    F: FnOnce(),
{
    /// Creates a guard that runs `closure` when dropped.
    pub fn new(closure: F) -> Self {
        Self {
            closure: Some(closure),
        }
    }

    /// Cancels the cleanup closure and consumes the guard.
    pub fn dismiss(mut self) {
        self.closure = None;
    }
}

impl<F> Drop for ScopeGuard<F>
where
    F: FnOnce(),
{
    fn drop(&mut self) {
        if let Some(closure) = self.closure.take() {
            closure();
        }
    }
}
