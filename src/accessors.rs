use crate::errors::IndexError;
use crate::Matrix;
use std::array;
use std::ops::{Index, IndexMut};

impl<T, const R: usize, const C: usize> Matrix<T, R, C> {
    pub const fn is_square(&self) -> bool {
        R == C
    }

    pub const fn size(&self) -> usize {
        R * C
    }
}

// get

impl<T, const R: usize, const C: usize> Matrix<T, R, C> {
    /// # Errors
    /// - row must index within bounds
    /// - col must index within bounds
    pub const fn get(&self, row: usize, col: usize) -> Result<&T, IndexError> {
        if row >= R && col >= C {
            return Err(IndexError::Both(row, col));
        }

        if row >= R {
            return Err(IndexError::Row(row));
        }

        if col >= C {
            return Err(IndexError::Column(col));
        }

        Ok(&self.0[row][col])
    }

    /// # Errors
    /// row must index within bounds
    pub fn get_row(&self, row: usize) -> Result<[&T; C], IndexError> {
        if row >= R {
            return Err(IndexError::Row(row));
        }

        Ok(array::from_fn(|col| &self.0[row][col]))
    }

    /// # Errors
    /// col must index within bounds
    pub fn get_col(&self, col: usize) -> Result<[&T; R], IndexError> {
        if col >= C {
            return Err(IndexError::Column(col));
        }

        Ok(array::from_fn(|row| &self.0[row][col]))
    }

    /// # Errors
    /// All elements of rows must index within bounds
    pub fn get_rows<I>(&self, rows: I) -> Result<Vec<[&T; C]>, IndexError>
    where
        I: IntoIterator<Item = usize>,
    {
        rows.into_iter().map(|row| self.get_row(row)).collect()
    }

    /// # Errors
    /// All elements of cols must index within bounds
    pub fn get_cols<I>(&self, cols: I) -> Result<Vec<[&T; R]>, IndexError>
    where
        I: IntoIterator<Item = usize>,
    {
        cols.into_iter().map(|col| self.get_col(col)).collect()
    }

    /// # Errors
    /// - All elements of rows must index within bounds
    /// - All elements of cols must index within bounds
    pub fn get_area<I1, I2>(&self, rows: I1, cols: I2) -> Result<Vec<Vec<&T>>, IndexError>
    where
        I1: IntoIterator<Item = usize>,
        I2: IntoIterator<Item = usize>,
        I2::IntoIter: Clone,
    {
        let cols = cols.into_iter();

        rows.into_iter()
            .map(|row| {
                cols.clone()
                    .map(|col| self.get(row, col))
                    .collect::<Result<Vec<_>, IndexError>>()
            })
            .collect::<Result<Vec<_>, IndexError>>()
    }
}

// get_unchecked

impl<T, const R: usize, const C: usize> Matrix<T, R, C> {
    pub const unsafe fn get_unchecked(&self, row: usize, col: usize) -> &T {
        debug_assert!(
            row < R && col < C,
            "Matrix::get_unchecked requires that the index is within the matrix"
        );

        let data_ptr = &raw const self.0;
        // SAFETY
        // Upheld by caller
        let row_ptr = unsafe { data_ptr.add(row) }.cast::<T>();
        // SAFETY
        // Upheld by caller
        unsafe { &*row_ptr.add(col) }
    }

    pub unsafe fn get_row_unchecked(&self, row: usize) -> [&T; C] {
        debug_assert!(
            row < R,
            "Matrix::get_row_unchecked requires that the index is within the matrix"
        );

        let data_ptr = (&raw const self.0).cast::<[T; C]>();
        // SAFETY
        // Upheld by caller
        let row_ptr = unsafe { data_ptr.add(row) }.cast::<T>();

        array::from_fn(|col| {
            // SAFETY
            // Upheld by caller
            unsafe { &*row_ptr.add(col) }
        })
    }

    pub unsafe fn get_col_unchecked(&self, col: usize) -> [&T; R] {
        debug_assert!(
            col < C,
            "Matrix::get_col_unchecked requires that the index is within the matrix"
        );

        let data_ptr = (&raw const self.0).cast::<T>();
        // SAFETY
        // Upheld by caller
        let col_ptr = unsafe { data_ptr.add(col) };

        array::from_fn(|row| {
            // SAFETY
            // Upheld by caller
            unsafe { &*col_ptr.add(row * C) }
        })
    }

    pub unsafe fn get_rows_unchecked<I>(&self, rows: I) -> Vec<[&T; C]>
    where
        I: IntoIterator<Item=usize>,
    {
        rows.into_iter()
            .map(|row| {
                debug_assert!(
                    row < R,
                    "Matrix::get_rows_unchecked requires that the index is within the matrix"
                );

                // SAFETY
                // Upheld by caller
                unsafe { self.get_row_unchecked(row) }
            })
            .collect()
    }

    pub unsafe fn get_cols_unchecked<I>(&self, cols: I) -> Vec<[&T; R]>
    where
        I: IntoIterator<Item=usize>,
    {
        cols.into_iter()
            .map(|col| {
                debug_assert!(
                    col < C,
                    "Matrix::get_cols_unchecked requires that the index is within the matrix"
                );

                // SAFETY
                // Upheld by caller
                unsafe { self.get_col_unchecked(col) }
            })
            .collect()
    }

    pub unsafe fn get_area_unchecked<I1, I2>(&self, rows: I1, cols: I2) -> Vec<Vec<&T>>
    where
        I1: IntoIterator<Item=usize>,
        I2: IntoIterator<Item=usize>,
        I2::IntoIter: Clone,
    {
        let cols = cols.into_iter();
        let data_ptr = (&raw const self.0).cast::<[T; C]>();

        rows.into_iter()
            .map(|row| {
                debug_assert!(
                    row < R,
                    "Matrix::get_area_unchecked requires that the index is within the matrix"
                );

                // SAFETY
                // Upheld by caller
                let row_ptr = unsafe { data_ptr.add(row) }.cast::<T>();

                cols.clone()
                    .map(|col| {
                        debug_assert!(
                            col < C,
                            "Matrix::get_area_unchecked requires that the index is within the matrix"
                        );

                        // SAFETY
                        // Upheld by caller
                        unsafe { &*row_ptr.add(col) }
                    })
                    .collect()
            })
            .collect()
    }
}

// get_mut

impl<T, const R: usize, const C: usize> Matrix<T, R, C> {
    /// # Errors
    /// - row must index within bounds
    /// - col must index within bounds
    pub const fn get_mut(&mut self, row: usize, col: usize) -> Result<&mut T, IndexError> {
        if row >= R && col >= C {
            return Err(IndexError::Both(row, col));
        }

        if row >= R {
            return Err(IndexError::Row(row));
        }

        if col >= C {
            return Err(IndexError::Column(col));
        }

        Ok(&mut self.0[row][col])
    }

    /// # Errors
    /// row must index within bounds
    pub fn get_mut_row(&mut self, row: usize) -> Result<[&mut T; C], IndexError> {
        let self_ptr = self as *mut Self;

        if row >= R {
            return Err(IndexError::Row(row));
        }

        Ok(array::from_fn(|col| {
            // Safety
            // Will always index different values and thus not make mutable aliases
            &mut unsafe { &mut *self_ptr }.0[row][col]
        }))
    }

    /// # Errors
    /// col must index within bounds
    pub fn get_mut_col(&mut self, col: usize) -> Result<[&mut T; R], IndexError> {
        let self_ptr = self as *mut Self;

        if col >= C {
            return Err(IndexError::Column(col));
        }

        Ok(array::from_fn(|row| {
            // Safety
            // Will always index different values and thus not make mutable aliases
            &mut unsafe { &mut *self_ptr }.0[row][col]
        }))
    }

    /// # Errors
    /// all elements of rows must index within bounds
    /// # Safety
    /// rows must have no duplicates
    pub unsafe fn get_mut_rows<I>(&mut self, rows: I) -> Result<Vec<[&mut T; C]>, IndexError>
    where
        I: IntoIterator<Item=usize>,
    {
        let self_ptr = self as *mut Self;

        rows.into_iter()
            .map(|row| {
                // Safety
                // Enforced by caller
                unsafe { &mut *self_ptr }.get_mut_row(row)
            })
            .collect()
    }

    /// # Errors
    /// all elements of cols must index within bounds
    /// # Safety
    /// cols must have no duplicates
    pub unsafe fn get_mut_cols<I>(&mut self, cols: I) -> Result<Vec<[&mut T; R]>, IndexError>
    where
        I: IntoIterator<Item=usize>,
    {
        let self_ptr = self as *mut Self;

        cols.into_iter()
            .map(|col| {
                // Safety
                // Enforced by caller
                unsafe { &mut *self_ptr }.get_mut_col(col)
            })
            .collect()
    }

    /// # Errors
    /// - all elements of rows must index within bounds
    /// - all elements of cols must index within bounds
    /// # Safety
    /// - rows must have no duplicates
    /// - cols must have no duplicates
    pub unsafe fn get_mut_area<I1, I2>(
        &mut self,
        rows: I1,
        cols: I2,
    ) -> Result<Vec<Vec<&mut T>>, IndexError>
    where
        I1: IntoIterator<Item=usize>,
        I2: IntoIterator<Item=usize>,
        I2::IntoIter: Clone,
    {
        let self_ptr = self as *mut Self;
        let cols = cols.into_iter();

        rows.into_iter()
            .map(|row| {
                cols.clone()
                    .map(|col| {
                        // Safety
                        // Enforced by caller
                        unsafe { &mut *self_ptr }.get_mut(row, col)
                    })
                    .collect::<Result<Vec<_>, IndexError>>()
            })
            .collect::<Result<Vec<_>, IndexError>>()
    }
}

// get_unchecked_mut

impl<T, const R: usize, const C: usize> Matrix<T, R, C> {
    pub const unsafe fn get_unchecked_mut(&mut self, row: usize, col: usize) -> &mut T {
        debug_assert!(
            row < R && col < C,
            "Matrix::get_unchecked_mut requires that the index is within the matrix"
        );

        let data_ptr = &raw mut self.0;
        // SAFETY
        // Upheld by caller
        let row_ptr = unsafe { data_ptr.add(row) }.cast::<T>();
        unsafe { &mut *row_ptr.add(col) }
    }

    pub unsafe fn get_row_unchecked_mut(&mut self, row: usize) -> [&mut T; C] {
        debug_assert!(
            row < R,
            "Matrix::get_row_unchecked_mut requires that the index is within the matrix"
        );

        let data_ptr = (&raw mut self.0).cast::<[T; C]>();
        // SAFETY
        // Upheld by caller
        let row_ptr = unsafe { data_ptr.add(row) }.cast::<T>();

        array::from_fn(|col| {
            // SAFETY
            // Upheld by caller
            unsafe { &mut *row_ptr.add(col) }
        })
    }

    pub unsafe fn get_col_unchecked_mut(&mut self, col: usize) -> [&mut T; R] {
        debug_assert!(
            col < C,
            "Matrix::get_col_unchecked_mut requires that the index is within the matrix"
        );

        let data_ptr = (&raw mut self.0).cast::<T>();
        // SAFETY
        // Upheld by caller
        let col_ptr = unsafe { data_ptr.add(col) };

        array::from_fn(|row| {
            // SAFETY
            // Upheld by caller
            unsafe { &mut *col_ptr.add(row * C) }
        })
    }

    /// # Safety
    /// rows must have no duplicates
    pub unsafe fn get_rows_unchecked_mut<I>(&mut self, rows: I) -> Vec<[&mut T; C]>
    where
        I: IntoIterator<Item=usize>,
    {
        let self_ptr = self as *mut Self;

        rows.into_iter()
            .map(|row| {
                debug_assert!(
                    row < R,
                    "Matrix::get_rows_unchecked_mut requires that the index is within the matrix"
                );

                unsafe { { &mut *self_ptr }.get_row_unchecked_mut(row) }
            })
            .collect()
    }

    /// # Safety
    /// cols must have no duplicates
    pub unsafe fn get_cols_unchecked_mut<I>(&mut self, cols: I) -> Vec<[&mut T; R]>
    where
        I: IntoIterator<Item=usize>,
    {
        let self_ptr = self as *mut Self;

        cols.into_iter()
            .map(|col| {
                debug_assert!(
                    col < C,
                    "Matrix::get_col_unchecked_mut requires that the index is within the matrix"
                );

                unsafe { { &mut *self_ptr }.get_col_unchecked_mut(col) }
            })
            .collect()
    }

    /// # Safety
    /// - rows must have no duplicates
    /// - cols must have no duplicates
    pub unsafe fn get_area_unchecked_mut<I1, I2>(&mut self, rows: I1, cols: I2) -> Vec<Vec<&mut T>>
    where
        I1: IntoIterator<Item=usize>,
        I2: IntoIterator<Item=usize>,
        I2::IntoIter: Clone,
    {
        let cols = cols.into_iter();
        let data_ptr = (&raw mut self.0).cast::<[T; C]>();

        rows.into_iter()
            .map(|row| {
                debug_assert!(
                    row < R,
                    "Matrix::get_area_unchecked_mut requires that the index is within the matrix"
                );

                let row_ptr = unsafe { data_ptr.add(row) }.cast::<T>();

                cols.clone()
                    .map(|col| {
                        debug_assert!(
                            col < C,
                            "Matrix::get_area_unchecked_mut requires that the index is within the matrix"
                        );

                        unsafe { &mut *row_ptr.add(col) }
                    })
                    .collect()
            })
            .collect()
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
