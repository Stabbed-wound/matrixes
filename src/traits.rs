use crate::Matrix;
use std::array;
use std::ops::Neg;

impl<T, const R: usize, const C: usize> Default for Matrix<T, R, C>
where
    T: Default,
{
    fn default() -> Self {
        Self(array::from_fn(|_| array::from_fn(|_| T::default())))
    }
}

impl<T, const R: usize, const C: usize> From<[[T; C]; R]> for Matrix<T, R, C> {
    fn from(value: [[T; C]; R]) -> Self {
        Self(value)
    }
}

impl<T, const R: usize, const C: usize> Neg for Matrix<T, R, C>
where
    T: Neg + Copy,
{
    type Output = Matrix<<T as Neg>::Output, R, C>;

    fn neg(self) -> Self::Output {
        self.map(|&elem| elem.neg())
    }
}

impl<T, const R: usize, const C: usize> Neg for &Matrix<T, R, C>
where
    T: Neg + Copy,
{
    type Output = Matrix<<T as Neg>::Output, R, C>;

    fn neg(self) -> Self::Output {
        self.map(|&elem| elem.neg())
    }
}
