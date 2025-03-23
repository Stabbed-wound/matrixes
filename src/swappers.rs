use crate::Matrix;
use crate::errors::IndexError;
use std::mem;

impl<T, const R: usize, const C: usize> Matrix<T, R, C> {
    /// # Errors
    /// - row1 must index within bounds
    /// - row2 must index within bounds
    pub fn swap_rows(&mut self, row1: usize, row2: usize) -> Result<(), IndexError> {
        if row1 == row2 {
            return Ok(());
        }

        let self_ptr = self as *mut Self;

        // Safety
        // row1 and row2 are different so this is safe
        let row1 = unsafe { &mut *self_ptr }.get_mut_row(row1)?;
        let row2 = unsafe { &mut *self_ptr }.get_mut_row(row2)?;

        row1.into_iter()
            .zip(row2)
            .for_each(|(elem1, elem2)| mem::swap(elem1, elem2));
        
        Ok(())
    }

    /// # Errors
    /// - col1 must index within bounds
    /// - col2 must index within bounds
    pub fn swap_cols(&mut self, col1: usize, col2: usize) -> Result<(), IndexError> {
        if col1 == col2 {
            return Ok(());
        }

        let self_ptr = self as *mut Self;

        // Safety
        // col1 and col2 are different so this is safe.
        let col1 = unsafe { &mut *self_ptr }.get_mut_col(col1)?;
        let col2 = unsafe { &mut *self_ptr }.get_mut_col(col2)?;

        col1.into_iter()
            .zip(col2)
            .for_each(|(elem1, elem2)| mem::swap(elem1, elem2));
        
        Ok(())
    }
}
