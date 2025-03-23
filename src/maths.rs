use crate::errors::IndexError;
use crate::Matrix;
use std::ops::{AddAssign, Mul, MulAssign};

impl<T, const R: usize, const C: usize> Matrix<T, R, C> {
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
