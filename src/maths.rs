use crate::{errors::IndexError, Matrix};
use num_traits::{ConstOne, ConstZero, One, Zero};
use std::{
    iter::zip,
    ops::{Add, AddAssign, Mul, MulAssign},
};

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

impl<T, const R: usize, const C: usize> Add for Matrix<T, R, C>
where
    T: Add,
{
    type Output = Matrix<<T as Add>::Output, R, C>;

    #[allow(clippy::op_ref)]
    fn add(self, rhs: Self) -> Self::Output {
        let rows_vec: Vec<_> = zip(self.0, rhs.0)
            .map(|(lhs, rhs)| {
                <[_; C]>::try_from(
                    zip(lhs, rhs)
                        .map(|(lhs, rhs)| lhs + rhs)
                        .collect::<Vec<_>>(),
                )
                .unwrap_or_else(|_| unreachable!())
            })
            .collect();

        Matrix::from(<[_; R]>::try_from(rows_vec).unwrap_or_else(|_| unreachable!()))
    }
}

impl<T, const R: usize, const C: usize> Add<&Self> for Matrix<T, R, C>
where
    T: Add + Copy,
{
    type Output = Matrix<<T as Add>::Output, R, C>;

    #[allow(clippy::op_ref)]
    fn add(self, rhs: &Self) -> Self::Output {
        &self + rhs
    }
}

impl<T, const R: usize, const C: usize> Add<Matrix<T, R, C>> for &Matrix<T, R, C>
where
    T: Add + Copy,
{
    type Output = Matrix<<T as Add>::Output, R, C>;

    #[allow(clippy::op_ref)]
    fn add(self, rhs: Matrix<T, R, C>) -> Self::Output {
        self + &rhs
    }
}

impl<T, const R: usize, const C: usize> Add for &Matrix<T, R, C>
where
    T: Add + Copy,
{
    type Output = Matrix<<T as Add>::Output, R, C>;

    fn add(self, rhs: Self) -> Self::Output {
        let rows_vec: Vec<_> = zip(&self.0, &rhs.0)
            .map(|(lhs, rhs)| {
                <[_; C]>::try_from(
                    zip(lhs, rhs)
                        .map(|(lhs, rhs)| *lhs + *rhs)
                        .collect::<Vec<_>>(),
                )
                .unwrap_or_else(|_| unreachable!())
            })
            .collect();

        Matrix::from(<[_; R]>::try_from(rows_vec).unwrap_or_else(|_| unreachable!()))
    }
}

impl<T, const R: usize, const S: usize, const C: usize> Mul<Matrix<T, S, C>> for Matrix<T, R, S>
where
    T: Mul<Output = T> + Add<Output = T> + Zero + Copy,
{
    type Output = Matrix<T, R, C>;

    fn mul(self, rhs: Matrix<T, S, C>) -> Self::Output {
        &self * &rhs
    }
}

impl<T, const R: usize, const S: usize, const C: usize> Mul<&Matrix<T, S, C>> for Matrix<T, R, S>
where
    T: Mul<Output = T> + Add<Output = T> + Zero + Copy,
{
    type Output = Matrix<T, R, C>;

    #[allow(clippy::op_ref)]
    fn mul(self, rhs: &Matrix<T, S, C>) -> Self::Output {
        &self * rhs
    }
}

impl<T, const R: usize, const S: usize, const C: usize> Mul<Matrix<T, S, C>> for &Matrix<T, R, S>
where
    T: Mul<Output = T> + Add<Output = T> + Zero + Copy,
{
    type Output = Matrix<T, R, C>;

    #[allow(clippy::op_ref)]
    fn mul(self, rhs: Matrix<T, S, C>) -> Self::Output {
        self * &rhs
    }
}

impl<T, const R: usize, const S: usize, const C: usize> Mul<&Matrix<T, S, C>> for &Matrix<T, R, S>
where
    T: Mul<Output = T> + Add<Output = T> + Zero + Copy,
{
    type Output = Matrix<T, R, C>;

    fn mul(self, rhs: &Matrix<T, S, C>) -> Self::Output {
        Matrix::from_fn(|i, j| {
            zip(self.get_row_unchecked(i), rhs.get_row_unchecked(j))
                .fold(T::zero(), |acc, (lhs, rhs)| acc + *lhs * *rhs)
        })
    }
}

impl<T, const R: usize, const C: usize> Zero for Matrix<T, R, C>
where
    T: Add<Output = T> + Zero,
{
    fn zero() -> Self {
        Self::from_fn(|_, _| T::zero())
    }

    fn is_zero(&self) -> bool {
        self.iter().all(T::is_zero)
    }
}

impl<T, const R: usize, const C: usize> ConstZero for Matrix<T, R, C>
where
    T: ConstZero + Copy,
{
    const ZERO: Self = Self([[T::ZERO; C]; R]);
}

impl<T, const N: usize> One for Matrix<T, N, N>
where
    T: Mul<Output = T> + Copy + Zero + One,
{
    fn one() -> Self {
        Self([[T::one(); N]; N])
    }
}

impl<T, const N: usize> ConstOne for Matrix<T, N, N>
where
    T: Mul<Output = T> + Copy + Zero + ConstOne,
{
    const ONE: Self = Self([[T::ONE; N]; N]);
}
