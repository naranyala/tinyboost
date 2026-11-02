use std::cell::Cell;

/// A scope guard that runs a closure when the guard is dropped.
///
/// This is a lightweight RAII pattern that helps ensure cleanup or finalization
/// occurs in a single, obvious place.
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
    pub fn new(closure: F) -> Self {
        Self {
            closure: Some(closure),
        }
    }

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
