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
