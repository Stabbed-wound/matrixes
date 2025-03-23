use crate::Matrix;
use std::{mem, slice, vec};

impl<T, const R: usize, const C: usize> Matrix<T, R, C> {
    pub fn iter(&self) -> <&Self as IntoIterator>::IntoIter {
        self.into_iter()
    }

    pub fn iter_mut(&mut self) -> <&mut Self as IntoIterator>::IntoIter {
        self.into_iter()
    }
}

impl<T, const R: usize, const C: usize> IntoIterator for Matrix<T, R, C> {
    type Item = T;
    type IntoIter = vec::IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        let flattened = self.0.as_flattened();
        let data = Vec::with_capacity(flattened.len());
        let data_ptr = &mut data.as_slice() as *mut &[T];
        // Safety
        // dst is valid and aligned
        unsafe { data_ptr.write(flattened) }
        mem::forget(self.0);

        data.into_iter()
    }
}

impl<'a, T, const R: usize, const C: usize> IntoIterator for &'a Matrix<T, R, C> {
    type Item = &'a T;
    type IntoIter = slice::Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.as_flattened().iter()
    }
}

impl<'a, T, const R: usize, const C: usize> IntoIterator for &'a mut Matrix<T, R, C> {
    type Item = &'a mut T;
    type IntoIter = slice::IterMut<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.as_flattened_mut().iter_mut()
    }
}
