use std::{ops::Deref, slice::Iter};

use crate::NonEmptyVec;

#[derive(Clone)]
pub struct NonEmptyIter<'a, T>(Iter<'a, T>);

impl<'a, T> NonEmptyIter<'a, T> {
    pub(crate) fn new_unchecked(iter: Iter<'a, T>) -> Self {
        NonEmptyIter(iter)
    }

    pub fn map<B, F>(self, f: F) -> NonEmptyMap<Self, F>
    where
        Self: Sized,
        F: FnMut(&T) -> B,
    {
        NonEmptyMap::new(self, f)
    }
}

pub struct NonEmptyMap<I, F> {
    iter: I,
    f: F,
}

impl<B, I: Iterator, F> Iterator for NonEmptyMap<I, F>
where
    F: FnMut(I::Item) -> B,
{
    type Item = B;

    #[inline]
    fn next(&mut self) -> Option<B> {
        self.iter.next().map(&mut self.f)
    }
}

impl<I, F> NonEmptyMap<I, F> {
    fn new(iter: I, f: F) -> NonEmptyMap<I, F> {
        NonEmptyMap { iter, f }
    }
}

impl<'a, A, B, F> NonEmptyMap<NonEmptyIter<'a, A>, F>
where
    F: FnMut(&A) -> B,
{
    pub fn collect(self) -> NonEmptyVec<B> {
        NonEmptyVec::try_from(self.iter.0.map(self.f).collect::<Vec<_>>()).unwrap()
    }
}

impl<'a, T> Iterator for NonEmptyIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }
}

impl<'a, T> Deref for NonEmptyIter<'a, T> {
    type Target = Iter<'a, T>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[cfg(test)]
mod tests {

    use crate::{non_empty_vec, NonEmptyVec};

    #[test]
    fn deref() {
        let vec = non_empty_vec![10, 20, 30, 40, 50];

        let iter = vec.iter();

        assert_eq!(iter.len(), 5);

        let result: Vec<i32> = iter.map(|&v| v).filter(|&v| v > 30).collect();

        assert_eq!(result, vec![40, 50]);
    }

    #[test]
    fn non_empty_collect() {
        let vec = non_empty_vec![10, 20, 30, 40, 50];

        let result: NonEmptyVec<_> = vec.iter().map(|v| v * 10).collect();

        assert_eq!(result, non_empty_vec![100, 200, 300, 400, 500]);

        let result: Vec<_> = vec.iter().map(|v| v * 10).filter(|&v| v > 300).collect();

        assert_eq!(result, vec![400, 500]);
    }
}
