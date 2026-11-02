use std::sync::OnceLock;

/// A one-time initialization value that is computed on first access.
pub struct LazyInit<T> {
    cell: OnceLock<T>,
    init: fn() -> T,
}

impl<T> LazyInit<T> {
    pub fn new(init: fn() -> T) -> Self {
        Self {
            cell: OnceLock::new(),
            init,
        }
    }

    pub fn get(&self) -> &T {
        self.cell.get_or_init(self.init)
    }

    pub fn into_inner(self) -> Option<T> {
        self.cell.into_inner()
    }
}
