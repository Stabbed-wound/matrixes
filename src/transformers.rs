use crate::errors::{IndexError, SquareError};
use crate::Matrix;
use std::{array, mem};

impl<T, const R: usize, const C: usize> Matrix<T, R, C> {
    pub fn map<U, F>(&self, mut f: F) -> Matrix<U, R, C>
    where
        F: FnMut(&T) -> U,
    {
        Matrix(array::from_fn(|row| {
            array::from_fn(|col| f(&self[(row, col)]))
        }))
    }

    /// # Errors
    /// - index1 must index within bounds
    /// - index2 must index within bounds
    pub fn swap_elems(
        &mut self,
        index1: (usize, usize),
        index2: (usize, usize),
    ) -> Result<(), IndexError> {
        if index1 == index2 {
            return Ok(());
        }

        let self_ptr = self as *mut Self;

        let elem1 = unsafe { &mut *self_ptr }.get_mut(index1.0, index1.1)?;
        let elem2 = unsafe { &mut *self_ptr }.get_mut(index2.0, index2.1)?;

        mem::swap(elem1, elem2);

        Ok(())
    }

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

    #[must_use]
    pub fn transpose(&self) -> Matrix<T, C, R>
    where
        T: Clone,
    {
        Matrix(array::from_fn(|row| {
            array::from_fn(|col| self[(col, row)].clone())
        }))
    }

    /// # Errors
    /// self must be a square matrix
    pub fn as_transpose(&mut self) -> Result<(), SquareError> {
        if !self.is_square() {
            return Err(SquareError::NotSquare);
        }

        let data_ptr = &raw mut self.0;

        for row in 0..R - 1 {
            for col in row + 1..C {
                // Safety
                // row and col are never equal so this is fine
                mem::swap(
                    &mut unsafe { &mut *data_ptr }[row][col],
                    &mut unsafe { &mut *data_ptr }[col][row],
                );
            }
        }

        Ok(())
    }
}
