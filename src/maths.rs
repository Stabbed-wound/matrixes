use crate::errors::IndexError;
use crate::Matrix;
use std::ops::{AddAssign, Mul, MulAssign};

impl<T, const R: usize, const C: usize> Matrix<T, R, C> {
    ///
    ///
    /// # Arguments
    ///
    /// * `source`:
    /// * `target`:
    ///
    /// returns: Result<(), `IndexError`>
    ///
    /// # Errors
    /// source and target must index within bounds
    ///
    /// # Examples
    ///
    /// ```
    ///
    /// ```
    pub fn add_rows(&mut self, source: usize, target: usize) -> Result<(), IndexError>
    where
        T: AddAssign + Copy,
    {
        let source_row = self.get_row(source)?.map(|elem| *elem);

        for (target, source) in self.get_mut_row(target)?.into_iter().zip(source_row) {
            *target += source;
        }

        Ok(())
    }

    /// # Errors
    /// source and target must index within bounds
    pub fn add_cols(&mut self, source: usize, target: usize) -> Result<(), IndexError>
    where
        T: AddAssign + Copy,
    {
        let source_col = self.get_col(source)?.map(|elem| *elem);

        for (target, source) in self.get_mut_col(target)?.into_iter().zip(source_col) {
            *target += source;
        }

        Ok(())
    }

    /// # Errors
    /// row must index within bounds
    pub fn scale_row(&mut self, row: usize, factor: T) -> Result<(), IndexError>
    where
        T: MulAssign + Copy,
    {
        for elem in self.get_mut_row(row)? {
            *elem *= factor;
        }

        Ok(())
    }

    /// # Errors
    /// col must index within bounds
    pub fn scale_col(&mut self, col: usize, factor: T) -> Result<(), IndexError>
    where
        T: MulAssign + Copy,
    {
        for elem in self.get_mut_col(col)? {
            *elem *= factor;
        }

        Ok(())
    }

    /// # Errors
    /// - source must index within bounds
    /// - target must index within bounds
    pub fn add_scaled_row(
        &mut self,
        source: usize,
        target: usize,
        factor: T,
    ) -> Result<(), IndexError>
    where
        T: AddAssign + Mul<Output = T> + Copy,
    {
        let scaled = self.get_row(source)?.map(|elem| *elem * factor);

        for (target, scaled) in self.get_mut_row(target)?.into_iter().zip(scaled) {
            *target += scaled;
        }

        Ok(())
    }

    /// # Errors
    /// - source must index within bounds
    /// - target must index within bounds
    pub fn add_scaled_col(
        &mut self,
        source: usize,
        target: usize,
        factor: T,
    ) -> Result<(), IndexError>
    where
        T: AddAssign + Mul<Output = T> + Copy,
    {
        let scaled = self.get_col(source)?.map(|elem| *elem * factor);

        for (target, scaled) in self.get_mut_col(target)?.into_iter().zip(scaled) {
            *target += scaled;
        }

        Ok(())
    }
}
