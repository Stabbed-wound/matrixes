use crate::Matrix;
use std::mem;

impl<T, const R: usize, const C: usize> Matrix<T, R, C> {
    pub fn swap_rows(&mut self, row1: usize, row2: usize) {
        if row1 == row2 {
            return;
        }

        let self_ptr = self as *mut Self;

        let Some(row1) = self.get_mut_row(row1) else {
            return;
        };
        // Safety
        // row1 and row2 are different so this is safe
        let Some(row2) = unsafe { &mut *self_ptr }.get_mut_row(row2) else {
            return;
        };

        row1.into_iter()
            .zip(row2)
            .for_each(|(elem1, elem2)| mem::swap(elem1, elem2));
    }

    pub fn swap_cols(&mut self, col1: usize, col2: usize) {
        if col1 == col2 {
            return;
        }

        let self_ptr = self as *mut Self;

        let Some(col1) = self.get_mut_col(col1) else {
            return;
        };
        // Safety
        // col1 and col2 are different so this is safe.
        let Some(col2) = unsafe { &mut *self_ptr }.get_mut_col(col2) else {
            return;
        };

        col1.into_iter()
            .zip(col2)
            .for_each(|(elem1, elem2)| mem::swap(elem1, elem2));
    }
}
