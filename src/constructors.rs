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

impl<T, const R: usize, const C: usize> From<[[T; C]; R]> for Matrix<T, R, C> {
    fn from(value: [[T; C]; R]) -> Self {
        Self(value)
    }
}

macro_rules! try_from_try_into_array {
    ($f:ty) => {
        impl<'a, T, const R: usize, const C: usize> TryFrom<$f> for Matrix<T, R, C>
        where
            [[T; C]; R]: TryFrom<$f>,
        {
            type Error = <[[T; C]; R] as TryFrom<$f>>::Error;
        
            fn try_from(value: $f) -> Result<Self, Self::Error> {
                Ok(Self(<[[T; C]; R]>::try_from(value)?))
            }
        }
    };
}

try_from_try_into_array!(&'a [[T; C]]);
try_from_try_into_array!(&'a mut [[T; C]]);
try_from_try_into_array!(Vec<[T; C]>);
