use crate::Matrix;
use num_traits::{One, Zero};
use std::array;

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
