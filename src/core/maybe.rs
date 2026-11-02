/// A small, explicit option-like wrapper for value-or-empty semantics.
///
/// This is intentionally tiny and ergonomically focused: it is easy to convert
/// from and to `Option<T>`, and it provides a few convenience methods that make
/// common fallback logic read more clearly.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Maybe<T> {
    Some(T),
    None,
}

impl<T> Maybe<T> {
    pub fn is_some(&self) -> bool {
        matches!(self, Self::Some(_))
    }

    pub fn is_none(&self) -> bool {
        matches!(self, Self::None)
    }

    pub fn map<U, F>(self, f: F) -> Maybe<U>
    where
        F: FnOnce(T) -> U,
    {
        match self {
            Self::Some(value) => Maybe::Some(f(value)),
            Self::None => Maybe::None,
        }
    }

    pub fn and_then<U, F>(self, f: F) -> Maybe<U>
    where
        F: FnOnce(T) -> Maybe<U>,
    {
        match self {
            Self::Some(value) => f(value),
            Self::None => Maybe::None,
        }
    }

    pub fn unwrap_or(self, default: T) -> T {
        match self {
            Self::Some(value) => value,
            Self::None => default,
        }
    }

    pub fn unwrap_or_default(self) -> T
    where
        T: Default,
    {
        match self {
            Self::Some(value) => value,
            Self::None => T::default(),
        }
    }

    /// Computes a fallback value from a closure only when `self` is `None`.
    pub fn unwrap_or_else(self, f: impl FnOnce() -> T) -> T {
        match self {
            Self::Some(value) => value,
            Self::None => f(),
        }
    }

    /// Converts `&Maybe<T>` into `Maybe<&T>` without consuming the original.
    pub fn as_ref(&self) -> Maybe<&T> {
        match self {
            Self::Some(value) => Maybe::Some(value),
            Self::None => Maybe::None,
        }
    }

    /// Converts `&mut Maybe<T>` into `Maybe<&mut T>` without consuming the
    /// original, which allows mutating the inner value in place.
    pub fn as_mut(&mut self) -> Maybe<&mut T> {
        match self {
            Self::Some(value) => Maybe::Some(value),
            Self::None => Maybe::None,
        }
    }

    /// Returns `true` if the value is `Some` and contains the given value.
    pub fn contains(&self, value: &T) -> bool
    where
        T: PartialEq<T>,
    {
        match self {
            Self::Some(inner) => inner == value,
            Self::None => false,
        }
    }
}

impl<T> From<Option<T>> for Maybe<T> {
    fn from(value: Option<T>) -> Self {
        match value {
            Some(value) => Self::Some(value),
            None => Self::None,
        }
    }
}

impl<T> From<Maybe<T>> for Option<T> {
    fn from(value: Maybe<T>) -> Self {
        match value {
            Maybe::Some(value) => Some(value),
            Maybe::None => None,
        }
    }
}
