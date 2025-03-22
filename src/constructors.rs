use crate::Matrix;
use num_traits::{One, Zero};
use std::array;

impl<T, const N: usize> Matrix<T, N, N> {
    #[must_use]
    pub fn new_identity() -> Self
    where
        T: Zero + One,
    {
        Self::new_from_function(|row, col| if row == col { T::one() } else { T::zero() })
    }
}

impl<T, const R: usize, const C: usize> Matrix<T, R, C> {
    pub const fn new_of_elem(elem: T) -> Self
    where
        T: Copy,
    {
        Self([[elem; C]; R])
    }

    pub fn new_from_function<F>(mut f: F) -> Self
    where
        F: FnMut(usize, usize) -> T,
    {
        Self(array::from_fn(|row| array::from_fn(|col| f(row, col))))
    }

    pub const fn new_from_arrays(data: [[T; C]; R]) -> Self {
        Self(data)
    }
}
