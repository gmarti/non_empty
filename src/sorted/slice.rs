use std::{fmt, ops::Deref};

use super::SortedVec;

#[derive(PartialEq, Eq)]
#[repr(transparent)]
pub struct SortedSlice<T> {
    inner: [T],
}

impl<T> SortedSlice<T> {
    pub(super) const unsafe fn new_unchecked(slice: &[T]) -> &SortedSlice<T> {
        // SAFETY: This type is `repr(transparent)`, so we can safely
        // cast the references like this.
        &*(slice as *const [T] as *const SortedSlice<T>)
    }

    pub(super) unsafe fn unchecked_boxed(slice: Box<[T]>) -> Box<Self> {
        // SAFETY: This type is `repr(transparent)`, so we can safely
        // cast the pointers like this.
        // `Box` does not necessarily have a guaranteed type layout
        // so it's safer to use methods to convert to/from raw pointers.
        let ptr = Box::into_raw(slice) as *mut Self;
        Box::from_raw(ptr)
    }

    pub fn as_slice(&self) -> &[T] {
        &self.inner
    }
}

impl<T> Default for Box<SortedSlice<T>> {
    fn default() -> Self {
        SortedVec::from_sorted_vec(vec![]).into_boxed_slice()
    }
}

impl<T: Clone> SortedSlice<T> {
    pub fn to_vec(&self) -> SortedVec<T> {
        SortedVec::from_sorted_vec(self.inner.to_vec())
    }
}

impl<T: Clone> Clone for Box<SortedSlice<T>> {
    fn clone(&self) -> Self {
        self.to_vec().into_boxed_slice()
    }
}

inner_debug!(SortedSlice);
inner_iterator!(SortedSlice);
inner_deref_slice!(SortedSlice);

#[cfg(test)]
mod tests {

    use super::*;
}
