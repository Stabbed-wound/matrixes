use num_traits::{One, Zero};
use std::ops::{Index, IndexMut, Neg};
use std::{array, mem};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Matrix<T, const R: usize, const C: usize>([[T; C]; R]);

// constructors

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

// getters

impl<T, const R: usize, const C: usize> Matrix<T, R, C> {
    pub const fn get(&self, row: usize, col: usize) -> Option<&T> {
        if row < R && col < C {
            return Some(&self.0[row][col]);
        }

        None
    }

    pub fn get_row(&self, row: usize) -> Option<[&T; C]> {
        if row < R {
            return Some(array::from_fn(|col| &self.0[row][col]));
        }

        None
    }

    pub fn get_col(&self, col: usize) -> Option<[&T; R]> {
        if col < C {
            return Some(array::from_fn(|row| &self.0[row][col]));
        }

        None
    }

    pub fn get_rows<I>(&self, rows: I) -> Option<Vec<[&T; C]>>
    where
        I: IntoIterator<Item = usize>,
    {
        rows.into_iter().map(|row| self.get_row(row)).collect()
    }

    pub fn get_cols<I>(&self, cols: I) -> Option<Vec<[&T; R]>>
    where
        I: IntoIterator<Item = usize>,
    {
        cols.into_iter().map(|col| self.get_col(col)).collect()
    }

    pub fn get_area<I1, I2>(&self, rows: I1, cols: I2) -> Option<Vec<Vec<&T>>>
    where
        I1: IntoIterator<Item = usize>,
        I2: IntoIterator<Item = usize>,
    {
        let mut cols = cols.into_iter();

        rows.into_iter()
            .map(|row| -> Option<Vec<&T>> {
                let row = self.0.get(row)?;

                cols.by_ref()
                    .map(|col| row.get(col))
                    .collect::<Option<Vec<&T>>>()
            })
            .collect::<Option<Vec<_>>>()
    }
}

// mut_getters

impl<T, const R: usize, const C: usize> Matrix<T, R, C> {
    pub fn get_mut(&mut self, row: usize, col: usize) -> Option<&mut T> {
        if row < R && col < C {
            return Some(&mut self.0[row][col]);
        }

        None
    }

    pub fn get_mut_row(&mut self, row: usize) -> Option<[&mut T; C]> {
        let self_ptr = self as *mut Self;

        if row < R {
            return Some(array::from_fn(|col| {
                // Safety
                // Will always index different values and thus not make mutable aliases
                &mut unsafe { &mut *self_ptr }.0[row][col]
            }));
        }

        None
    }

    pub fn get_mut_col(&mut self, col: usize) -> Option<[&mut T; R]> {
        let self_ptr = self as *mut Self;

        if col < C {
            return Some(array::from_fn(|row| {
                // Safety
                // Will always index different values and thus not make mutable aliases
                &mut unsafe { &mut *self_ptr }.0[row][col]
            }));
        }

        None
    }

    /// # Safety
    /// rows must have no duplicates
    pub unsafe fn get_mut_rows<I>(&mut self, rows: I) -> Option<Vec<[&mut T; C]>>
    where
        I: IntoIterator<Item = usize>,
    {
        let self_ptr = self as *mut Self;

        rows.into_iter()
            .map(|row| unsafe { &mut *self_ptr }.get_mut_row(row))
            .collect()
    }

    /// # Safety
    /// cols must have no duplicates
    pub unsafe fn get_mut_cols<I>(&mut self, cols: I) -> Option<Vec<[&mut T; R]>>
    where
        I: IntoIterator<Item = usize>,
    {
        let self_ptr = self as *mut Self;

        cols.into_iter()
            .map(|col| unsafe { &mut *self_ptr }.get_mut_col(col))
            .collect()
    }

    /// # Safety
    /// - rows must have no duplicates
    /// - cols must have no duplicates
    pub unsafe fn get_mut_area<I1, I2>(&mut self, rows: I1, cols: I2) -> Option<Vec<Vec<&mut T>>>
    where
        I1: IntoIterator<Item = usize>,
        I2: IntoIterator<Item = usize>,
    {
        let mut cols = cols.into_iter();

        rows.into_iter()
            .map(|row| -> Option<Vec<&mut T>> {
                let row_ptr = self.0.get_mut(row)? as *mut [T; C];

                cols.by_ref()
                    .map(|col| unsafe { &mut *row_ptr }.get_mut(col))
                    .collect::<Option<Vec<&mut T>>>()
            })
            .collect::<Option<Vec<Vec<&mut T>>>>()
    }
}

// swappers

impl<T, const R: usize, const C: usize> Matrix<T, R, C> {
    pub fn swap_rows(&mut self, row1: usize, row2: usize) {
        if row1 == row2 {
            return;
        }

        let self_ptr = self as *mut Self;

        let Some(row1) = self.get_mut_row(row1) else {
            return;
        };
        // Safety
        // row1 and row2 are different so this is safe
        let Some(row2) = unsafe { &mut *self_ptr }.get_mut_row(row2) else {
            return;
        };

        row1.into_iter()
            .zip(row2)
            .for_each(|(elem1, elem2)| mem::swap(elem1, elem2));
    }

    pub fn swap_cols(&mut self, col1: usize, col2: usize) {
        if col1 == col2 {
            return;
        }

        let self_ptr = self as *mut Self;

        let Some(col1) = self.get_mut_col(col1) else {
            return;
        };
        // Safety
        // col1 and col2 are different so this is safe.
        let Some(col2) = unsafe { &mut *self_ptr }.get_mut_col(col2) else {
            return;
        };

        col1.into_iter()
            .zip(col2)
            .for_each(|(elem1, elem2)| mem::swap(elem1, elem2));
    }
}

// transformers

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
