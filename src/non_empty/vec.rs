use std::{fmt, ops::Deref};

use super::slice::NonEmptySlice;

#[derive(Clone, PartialEq, Eq)]
#[repr(transparent)]
pub struct NonEmptyVec<T> {
    inner: Vec<T>,
}

mod error {
    #[derive(Debug)]
    pub struct Empty;
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


inner_vec_iterator!(NonEmptyVec);
inner_iterator!(NonEmptyVec);
inner_debug!(NonEmptyVec);

impl<T> Deref for NonEmptyVec<T> {
    type Target = NonEmptySlice<T>;

    #[inline]
    fn deref(&self) -> &Self::Target {
        self.as_non_empty_slice()
    }
}

#[macro_export]
macro_rules! non_empty_vec {
   ($($x:expr),+ $(,)?) => {{
        NonEmptyVec::try_from(vec![$($x),+]).unwrap()
   }};
    ($h:expr) => {
        NonEmptyVec::one($h)
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
}
