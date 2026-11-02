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
