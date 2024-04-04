use std::{
    fmt,
    num::NonZeroUsize,
    ops::{Deref, DerefMut},
};

use super::slice::NonEmptySlice;

#[derive(Clone, PartialEq, Eq)]
pub struct NonEmptyVec<T> {
    inner: Vec<T>,
}

mod error {
    use std::{error::Error, fmt};

    #[derive(Debug)]
    pub struct Empty;

    impl fmt::Display for Empty {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "empty vec")
        }
    }

    impl Error for Empty {}
}

impl<T> NonEmptyVec<T> {
    pub fn one(first: T) -> NonEmptyVec<T> {
        NonEmptyVec { inner: vec![first] }
    }

    pub fn with_capacity(first: T, capacity: usize) -> NonEmptyVec<T> {
        let mut inner = Vec::with_capacity(capacity);
        inner.push(first);
        NonEmptyVec { inner }
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

    pub fn push(&mut self, value: T) {
        self.inner.push(value)
    }

    pub fn reverse(&mut self) {
        self.inner.reverse()
    }

    pub fn split_first(&self) -> (&T, &[T]) {
        (self.first(), self.tail())
    }

    pub fn split_last(&self) -> (&[T], &T) {
        (self.init(), self.last())
    }

    pub fn as_non_empty_slice(&self) -> &NonEmptySlice<T> {
        unsafe { NonEmptySlice::new_unchecked(&self.inner) }
    }

    pub fn as_non_empty_slice_mut(&mut self) -> &mut NonEmptySlice<T> {
        unsafe { NonEmptySlice::new_unchecked_mut(&mut self.inner) }
    }

    pub fn as_slice(&self) -> &[T] {
        &self.inner
    }

    pub fn as_vec(&self) -> &Vec<T> {
        &self.inner
    }

    pub fn into_vec(self) -> Vec<T> {
        self.inner
    }

    pub fn into_boxed_slice(self) -> Box<NonEmptySlice<T>> {
        let b = self.inner.into_boxed_slice();
        unsafe { NonEmptySlice::unchecked_boxed(b) }
    }

    pub fn truncate(&mut self, len: NonZeroUsize) {
        self.inner.truncate(len.get())
    }
}

impl<T: PartialEq> NonEmptyVec<T> {
    pub fn dedup(&mut self) {
        self.inner.dedup();
    }
}

impl<T: Clone> NonEmptyVec<T> {
    pub fn from_init_last(init: &[T], last: T) -> NonEmptyVec<T> {
        let mut inner = Vec::with_capacity(init.len() + 1);
        inner.extend_from_slice(init);
        inner.push(last);
        NonEmptyVec { inner }
    }

    pub fn from_first_tail(first: T, tail: &[T]) -> NonEmptyVec<T> {
        let mut inner = Vec::with_capacity(tail.len() + 1);
        inner.push(first);
        inner.extend_from_slice(tail);
        NonEmptyVec { inner }
    }

    pub fn extend_from_slice(&mut self, other: &[T]) {
        self.inner.extend_from_slice(other)
    }
}

impl<'a, T> Extend<&'a T> for NonEmptyVec<T>
where
    T: 'a + Copy,
{
    fn extend<I: IntoIterator<Item = &'a T>>(&mut self, iter: I) {
        self.inner.extend(iter);
    }
}

impl<T> Extend<T> for NonEmptyVec<T> {
    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        self.inner.extend(iter);
    }
}

impl<T: Clone> From<&NonEmptySlice<T>> for NonEmptyVec<T> {
    fn from(slice: &NonEmptySlice<T>) -> Self {
        slice.to_non_empty_vec()
    }
}

impl<T> TryFrom<Vec<T>> for NonEmptyVec<T> {
    type Error = error::Empty;

    fn try_from(vec: Vec<T>) -> Result<Self, Self::Error> {
        if vec.is_empty() {
            Err(error::Empty)
        } else {
            Ok(NonEmptyVec { inner: vec })
        }
    }
}

impl<T: fmt::Debug> fmt::Debug for NonEmptyVec<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&*self.inner, f)
    }
}

impl<T> IntoIterator for NonEmptyVec<T> {
    type Item = T;
    type IntoIter = std::vec::IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        self.inner.into_iter()
    }
}

impl<'a, T> IntoIterator for &'a NonEmptyVec<T> {
    type Item = &'a T;
    type IntoIter = std::slice::Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<T> Deref for NonEmptyVec<T> {
    type Target = NonEmptySlice<T>;

    #[inline]
    fn deref(&self) -> &Self::Target {
        self.as_non_empty_slice()
    }
}

impl<T> DerefMut for NonEmptyVec<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.as_non_empty_slice_mut()
    }
}

#[macro_export]
macro_rules! non_empty_vec {
   ($($x:expr),+ $(,)?) => {{
        $crate::NonEmptyVec::try_from(vec![$($x),+]).unwrap()
   }};
    ($h:expr) => {
        $crate::NonEmptyVec::one($h)
    };
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn one() {
        let non_empty_vec = NonEmptyVec::one(10);
        assert_eq!(non_empty_vec.len(), 1);
        assert_eq!(non_empty_vec.first(), &10);
        assert_eq!(non_empty_vec.last(), &10);
        assert!(non_empty_vec.init().is_empty());
        assert!(non_empty_vec.tail().is_empty());
    }

    #[test]
    fn push() {
        let mut non_empty_vec = NonEmptyVec::one(10);

        non_empty_vec.push(20);

        assert_eq!(non_empty_vec.len(), 2);
        assert_eq!(non_empty_vec.first(), &10);
        assert_eq!(non_empty_vec.last(), &20);
        assert_eq!(non_empty_vec.init(), &[10]);
        assert_eq!(non_empty_vec.tail(), &[20]);

        non_empty_vec.push(30);

        assert_eq!(non_empty_vec.len(), 3);
        assert_eq!(non_empty_vec.first(), &10);
        assert_eq!(non_empty_vec.last(), &30);
        assert_eq!(non_empty_vec.init(), &[10, 20]);
        assert_eq!(non_empty_vec.tail(), &[20, 30]);
    }

    #[test]
    fn non_empty_vec_macro() {
        let one = non_empty_vec![10];

        assert_eq!(one.len(), 1);
        assert_eq!(one.first(), &10);
        assert_eq!(one.last(), &10);
        assert!(one.init().is_empty());
        assert!(one.tail().is_empty());

        let multiple = non_empty_vec![10, 20, 30, 40, 50];

        assert_eq!(multiple.len(), 5);
        assert_eq!(multiple.first(), &10);
        assert_eq!(multiple.last(), &50);
        assert_eq!(multiple.init(), &[10, 20, 30, 40]);
        assert_eq!(multiple.tail(), &[20, 30, 40, 50]);
    }

    #[test]
    fn debug() {
        let multiple = non_empty_vec![10, 20, 30, 40, 50];

        let result = format!("{multiple:?}");
        assert_eq!(result, "[10, 20, 30, 40, 50]");
    }

    #[test]
    fn reverse() {
        let mut multiple = non_empty_vec![10, 20, 30, 40, 50];
        let reverse = non_empty_vec![50, 40, 30, 20, 10];

        multiple.reverse();

        assert_eq!(multiple, reverse);
    }

    #[test]
    fn split() {
        let multiple = non_empty_vec![10, 20, 30, 40, 50];

        assert_eq!(multiple.split_first(), (&10, &[20, 30, 40, 50][..]));
        assert_eq!(multiple.split_last(), (&[10, 20, 30, 40][..], &50));
    }

    #[test]
    fn extend_from_slice() {
        let mut one = non_empty_vec![10];
        let multiple = non_empty_vec![10, 20, 30, 40, 50];
        one.extend_from_slice(&[20, 30, 40, 50]);

        assert_eq!(one, multiple);
    }

    #[test]
    fn extend() {
        let mut one = non_empty_vec![10];
        let multiple = non_empty_vec![10, 20, 30, 40, 50];
        one.extend(multiple.iter());

        assert_eq!(one, non_empty_vec![10, 10, 20, 30, 40, 50]);
    }

    #[test]
    fn non_empty_vec_of_simple_struct() {
        // No clone, no PartialEq, no Eq
        struct Test(usize);

        let non_empty_vec = non_empty_vec![Test(0)];

        assert!(non_empty_vec.first().0 == 0);
    }

    #[test]
    fn truncate() {
        let mut v = non_empty_vec![1, 2];
        v.truncate(NonZeroUsize::new(1).unwrap());
        assert_eq!(v, non_empty_vec![1]);

        let mut v = non_empty_vec![1, 2];
        v.truncate(NonZeroUsize::new(2).unwrap());
        assert_eq!(v, non_empty_vec![1, 2]);

        let mut v = non_empty_vec![1, 2, 3];
        v.truncate(NonZeroUsize::new(2).unwrap());
        assert_eq!(v, non_empty_vec![1, 2]);
    }

    #[test]
    fn dedup() {
        let mut v = non_empty_vec![1, 2];
        v.dedup();
        assert_eq!(v, non_empty_vec![1, 2]);

        let mut v = non_empty_vec![1, 1];
        v.dedup();
        assert_eq!(v, non_empty_vec![1]);

        let mut v = non_empty_vec![1];
        v.dedup();
        assert_eq!(v, non_empty_vec![1]);

        let mut v = non_empty_vec![1, 2, 1];
        v.dedup();
        assert_eq!(v, non_empty_vec![1, 2, 1]);
    }
}
