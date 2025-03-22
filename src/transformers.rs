use crate::Matrix;
use std::array;

impl<T, const R: usize, const C: usize> Matrix<T, R, C> {
    pub fn map<U, F>(&self, mut f: F) -> Matrix<U, R, C>
    where
        F: FnMut(&T) -> U,
    {
        Matrix(array::from_fn(|row| {
            array::from_fn(|col| f(&self[(row, col)]))
        }))
    }

    pub fn for_each<F>(&mut self, mut f: F)
    where
        F: FnMut(&mut T),
    {
        self.0.iter_mut().for_each(|row| {
            row.iter_mut().for_each(&mut f);
        });
    }
}
