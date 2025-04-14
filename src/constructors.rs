use crate::errors::TryFromSlicesError;
use crate::Matrix;
use num_traits::{One, Zero};
use std::array;
use std::ops::Deref;

impl<T, const R: usize, const C: usize> Matrix<T, R, C> {
    #[must_use]
    pub fn new() -> Self
    where
        T: Default,
    {
        Self::default()
    }

    pub const fn of_elem(elem: T) -> Self
    where
        T: Copy,
    {
        Self([[elem; C]; R])
    }

    pub fn from_fn<F>(mut f: F) -> Self
    where
        F: FnMut(usize, usize) -> T,
    {
        Self(array::from_fn(|row| array::from_fn(|col| f(row, col))))
    }
}

impl<T, const N: usize> Matrix<T, N, N> {
    #[must_use]
    pub fn identity() -> Self
    where
        T: Zero + One,
    {
        Self::from_fn(|row, col| if row == col { T::one() } else { T::zero() })
    }
}

impl<T, const R: usize, const C: usize> From<[[T; C]; R]> for Matrix<T, R, C> {
    fn from(value: [[T; C]; R]) -> Self {
        Self(value)
    }
}

impl<T, I, const R: usize, const C: usize> TryFrom<&[I]> for Matrix<T, R, C>
where
    I: Deref<Target = [T]>,
    T: Copy,
{
    type Error = TryFromSlicesError;

    fn try_from(value: &[I]) -> Result<Self, Self::Error> {
        if value.len() != R {
            return Err(TryFromSlicesError::Rows(value.len()));
        }

        let rows: Result<Vec<_>, TryFromSlicesError> = value
            .iter()
            .enumerate()
            .map(|(i, row)| {
                <[T; C]>::try_from(&**row).map_err(|_| TryFromSlicesError::Columns(row.len(), i))
            })
            .collect();

        rows.map(|rows| Self(<[_; R]>::try_from(rows).unwrap_or_else(|_| unreachable!())))
    }
}
