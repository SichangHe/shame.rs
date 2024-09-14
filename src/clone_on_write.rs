use super::{prelude::*, *};

/// Owned clone-on-write convenience smart pointer.
/// Cloning a `CW` only clones the [`Arc`] it wraps, not the inner value.
/// To clone the inner value, use [`CW::inner_clone`].
///
/// Taking a mutable reference to a `CW` clones the inner value if it is shared.
///
/// Since weak reference handling is not implemented,
/// circular references should be avoided because they cause memory leaks.
#[derive(Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[derive_where(Clone)]
pub struct CW<T> {
    inner: Arc<T>,
}

impl<T: Clone> CW<T> {
    /// Actually clone the inner value.
    pub fn inner_clone(&self) -> Self {
        Arc::new((*self.inner).clone()).into()
    }

    /// Take the inner value and consume the `CW`.
    /// If the inner value is shared, it is cloned.
    pub fn take(self) -> T {
        Arc::unwrap_or_clone(self.inner)
    }
}

impl<T> Deref for CW<T> {
    type Target = T;
    fn deref(&self) -> &T {
        &self.inner
    }
}

impl<T: Clone> DerefMut for CW<T> {
    fn deref_mut(&mut self) -> &mut T {
        Arc::make_mut(&mut self.inner)
    }
}

impl<T> CW<T> {
    pub fn inner(&self) -> &Arc<T> {
        &self.inner
    }
}

impl<T> From<T> for CW<T> {
    fn from(data: T) -> Self {
        Arc::new(data).into()
    }
}

impl<T> From<Arc<T>> for CW<T> {
    fn from(inner: Arc<T>) -> Self {
        Self { inner }
    }
}

impl<T> From<CW<T>> for Arc<T> {
    fn from(val: CW<T>) -> Self {
        val.inner
    }
}

#[cfg(test)]
mod tests;
