macro_rules! inner_vec_iterator {
    ($type_name:ident) => {      
        impl<T> IntoIterator for $type_name<T> {
            type Item = T;
            type IntoIter = std::vec::IntoIter<T>;
        
            fn into_iter(self) -> Self::IntoIter {
                self.inner.into_iter()
            }
        }
    };
}

macro_rules! inner_iterator {
    ($type_name:ident) => {
             
        impl<'a, T> IntoIterator for &'a $type_name<T> {
            type Item = &'a T;
            type IntoIter = std::slice::Iter<'a, T>;
        
            fn into_iter(self) -> Self::IntoIter {
                self.iter()
            }
        }
    };
}

macro_rules! inner_debug {
    ($type_name:ident) => {      
        impl<T: fmt::Debug> fmt::Debug for $type_name<T> {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                fmt::Debug::fmt(&self.inner, f)
            }
        }
    };
}

macro_rules! inner_deref_slice {
    ($type_name:ident) => {      
        impl<T> Deref for $type_name<T> {
            type Target = [T];
        
            #[inline]
            fn deref(&self) -> &Self::Target {
                &self.inner
            }
        }
    };
}


