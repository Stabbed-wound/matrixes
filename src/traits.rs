use crate::Matrix;
use std::array;
use std::ops::{Index, IndexMut, Neg};

impl<T: Default, const R: usize, const C: usize> Default for Matrix<T, R, C> {
    fn default() -> Self {
        Self(array::from_fn(|_| array::from_fn(|_| T::default())))
    }
}

impl<T, const R: usize, const C: usize> Index<(usize, usize)> for Matrix<T, R, C> {
    type Output = T;

    fn index(&self, (row, col): (usize, usize)) -> &Self::Output {
        &self.0[row][col]
    }
}

impl<T, const R: usize, const C: usize> IndexMut<(usize, usize)> for Matrix<T, R, C> {
    fn index_mut(&mut self, (row, col): (usize, usize)) -> &mut Self::Output {
        &mut self.0[row][col]
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
