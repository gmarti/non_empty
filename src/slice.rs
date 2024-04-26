use std::{fmt, num::NonZeroUsize, ops::Deref};

use super::NonEmptyVec;

#[derive(PartialEq, Eq)]
#[repr(transparent)]
pub struct NonEmptySlice<T> {
    inner: [T],
}

mod error {
    use std::{error::Error, fmt};

    #[derive(Debug)]
    pub struct Empty;

    impl fmt::Display for Empty {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "empty slice")
        }
    }

    impl Error for Empty {}
}

impl<T> NonEmptySlice<T> {
    pub fn try_from_slice(slice: &[T]) -> Result<&NonEmptySlice<T>, error::Empty> {
        if !slice.is_empty() {
            Ok(unsafe { NonEmptySlice::new_unchecked(slice) })
        } else {
            Err(error::Empty)
        }
    }

    pub(super) const unsafe fn new_unchecked(slice: &[T]) -> &NonEmptySlice<T> {
        debug_assert!(!slice.is_empty());
        // SAFETY: This type is `repr(transparent)`, so we can safely
        // cast the references like this.
        &*(slice as *const [T] as *const NonEmptySlice<T>)
    }

    pub(super) unsafe fn new_unchecked_mut(slice: &mut [T]) -> &mut NonEmptySlice<T> {
        debug_assert!(!slice.is_empty());
        // SAFETY: This type is `repr(transparent)`, so we can safely
        // cast the references like this.
        &mut *(slice as *mut [T] as *mut NonEmptySlice<T>)
    }

    pub(super) unsafe fn unchecked_boxed(slice: Box<[T]>) -> Box<Self> {
        debug_assert!(!slice.is_empty());
        // SAFETY: This type is `repr(transparent)`, so we can safely
        // cast the pointers like this.
        // `Box` does not necessarily have a guaranteed type layout
        // so it's safer to use methods to convert to/from raw pointers.
        let ptr = Box::into_raw(slice) as *mut Self;
        Box::from_raw(ptr)
    }

    pub fn non_zero_len(&self) -> NonZeroUsize {
        self.inner.len().try_into().unwrap()
    }

    pub fn first(&self) -> &T {
        &self.inner[0]
    }

    pub fn tail(&self) -> &[T] {
        &self.inner[1..]
    }

    pub fn last(&self) -> &T {
        &self.inner[self.len() - 1]
    }

    pub fn init(&self) -> &[T] {
        &self.inner[..self.len() - 1]
    }

    pub fn split_first(&self) -> (&T, &[T]) {
        (self.first(), self.tail())
    }

    pub fn split_last(&self) -> (&[T], &T) {
        (self.init(), self.last())
    }

    pub fn as_slice(&self) -> &[T] {
        &self.inner
    }

    pub fn reverse(&mut self) {
        self.inner.reverse()
    }
}

impl<T: Clone> NonEmptySlice<T> {
    pub fn to_non_empty_vec(&self) -> NonEmptyVec<T> {
        self.inner.to_vec().try_into().unwrap()
    }
}

impl<T: Clone> Clone for Box<NonEmptySlice<T>> {
    fn clone(&self) -> Self {
        self.to_non_empty_vec().into_boxed_slice()
    }
}

impl<T: fmt::Debug> fmt::Debug for NonEmptySlice<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&self.inner, f)
    }
}

impl<'a, T> IntoIterator for &'a NonEmptySlice<T> {
    type Item = &'a T;
    type IntoIter = std::slice::Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<T> Deref for NonEmptySlice<T> {
    type Target = [T];

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<'a, T> TryFrom<&'a [T]> for &'a NonEmptySlice<T> {
    type Error = error::Empty;

    #[inline]
    fn try_from(value: &'a [T]) -> Result<Self, Self::Error> {
        NonEmptySlice::try_from_slice(value)
    }
}

impl<T> TryFrom<Box<[T]>> for Box<NonEmptySlice<T>> {
    type Error = error::Empty;

    fn try_from(value: Box<[T]>) -> Result<Self, Self::Error> {
        if !value.is_empty() {
            // SAFETY: We just checked that it's not empty,
            // so we can safely create a `NonEmptySlice`.
            Ok(unsafe { NonEmptySlice::unchecked_boxed(value) })
        } else {
            Err(error::Empty)
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::non_empty_vec;

    #[test]
    fn from_non_empty_vec() {
        let non_empty_slice: &NonEmptySlice<i32> = &non_empty_vec![10, 20, 30, 40, 50];

        assert_eq!(non_empty_slice.as_slice(), &[10, 20, 30, 40, 50])
    }

    #[test]
    fn debug() {
        let multiple: &NonEmptySlice<i32> = &non_empty_vec![10, 20, 30, 40, 50];

        let result = format!("{multiple:?}");
        assert_eq!(result, "[10, 20, 30, 40, 50]");
    }

    #[test]
    fn split() {
        let multiple: &NonEmptySlice<i32> = &non_empty_vec![10, 20, 30, 40, 50];

        assert_eq!(multiple.split_first(), (&10, &[20, 30, 40, 50][..]));
        assert_eq!(multiple.split_last(), (&[10, 20, 30, 40][..], &50));
    }

    #[test]
    fn reverse() {
        let multiple: &mut NonEmptySlice<_> = &mut non_empty_vec![10, 20, 30, 40, 50];
        let reverse: &NonEmptySlice<_> = &non_empty_vec![50, 40, 30, 20, 10];

        multiple.reverse();

        assert_eq!(multiple, reverse);
    }

    #[test]
    fn non_empty_slice_of_simple_struct() {
        // No clone, no PartialEq, no Eq
        struct Test(usize);

        let non_empty_slice: &NonEmptySlice<Test> = &non_empty_vec![Test(0)];

        assert!(non_empty_slice.first().0 == 0);
    }

    #[test]
    fn new() -> Result<(), error::Empty> {
        let vec = vec![10, 20, 30];
        let result = NonEmptySlice::try_from_slice(&vec)?;

        assert_eq!(result.len(), 3);
        assert_eq!(result[0], 10);
        assert_eq!(result[1], 20);
        assert_eq!(result.first(), &10);
        assert_eq!(result.last(), &30);

        let vec: Vec<i32> = Vec::new();
        let result = NonEmptySlice::try_from_slice(&vec);

        assert!(result.is_err());

        Ok(())
    }

    #[test]
    fn try_from_slice() -> Result<(), error::Empty> {
        let vec = [10, 20, 30];
        let result: &NonEmptySlice<i32> = vec[..].try_into()?;

        assert_eq!(result.len(), 3);
        assert_eq!(result[0], 10);
        assert_eq!(result[1], 20);
        assert_eq!(result.first(), &10);
        assert_eq!(result.last(), &30);

        let vec: Vec<i32> = Vec::new();
        let result: Result<&NonEmptySlice<i32>, _> = vec[..].try_into();

        assert!(result.is_err());

        Ok(())
    }
}
