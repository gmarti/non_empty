use std::{fmt, ops::Deref};

use super::slice::SortedSlice;



#[derive(Clone, PartialEq, Eq)]
pub struct SortedVec<K, T> {
    inner: Box<[T]>,
    by : Box<dyn Fn(T) -> K>
}

impl<T> SortedVec<T> {
    
    pub fn empty() -> SortedVec<T> {
        SortedVec {
            inner: Box::new([]),
        }
    }

    pub fn as_sorted_slice(&self) -> &SortedSlice<T> {
        unsafe { SortedSlice::new_unchecked(&self.inner) }
    }

    pub fn as_slice(&self) -> &[T] {
        &self.inner
    }

    pub fn into_vec(self) -> Vec<T> {
        self.inner.into_vec()
    }

    pub fn into_boxed_slice(self) -> Box<SortedSlice<T>> {
        unsafe { SortedSlice::unchecked_boxed(self.inner) }
    }

    pub(super) fn from_sorted_vec(vec: Vec<T>) -> SortedVec<T> {
        SortedVec {
            inner: vec.into_boxed_slice(),
        }
    }
}

impl<T: PartialEq + PartialOrd + Ord> SortedVec<T> {
    pub fn sort_vec(mut vec: Vec<T>) -> SortedVec<T> {
        vec.sort_unstable();
        vec.dedup();
        SortedVec {
            inner: vec.into_boxed_slice(),
            by : T -> T
        }
    }

    pub fn sort_vec_by(mut vec: Vec<T>, by : T -> K) -> SortedVec<T> {
        vec.sort_unstable();
        vec.dedup();
        SortedVec {
            inner: vec.into_boxed_slice(),
            by
        }
    }
}

inner_iterator!(SortedVec);
inner_debug!(SortedVec);

impl<T> IntoIterator for SortedVec<T> {
    type Item = T;
    type IntoIter = std::vec::IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        self.inner.into_vec().into_iter()
    }
}

impl<T> Deref for SortedVec<T> {
    type Target = SortedSlice<T>;

    #[inline]
    fn deref(&self) -> &Self::Target {
        self.as_sorted_slice()
    }
}

#[cfg(test)]
mod tests {

    use super::*;
}
