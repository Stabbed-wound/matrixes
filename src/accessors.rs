use crate::Matrix;
use std::array;

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
