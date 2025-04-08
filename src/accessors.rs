use crate::errors::IndexError;
use crate::Matrix;
use std::array;

impl<T, const R: usize, const C: usize> Matrix<T, R, C> {
    pub const fn is_square(&self) -> bool {
        R == C
    }

    pub const fn size(&self) -> usize {
        R * C
    }

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
                let row = self.0.get(row).ok_or(IndexError::Row(row))?;

                cols.clone()
                    .map(|col| row.get(col).ok_or(IndexError::Column(col)))
                    .collect::<Result<Vec<&T>, IndexError>>()
            })
            .collect::<Result<Vec<_>, IndexError>>()
    }
}

impl<T, const R: usize, const C: usize> Matrix<T, R, C> {
    /// # Errors
    /// - row must index within bounds
    /// - col must index within bounds
    pub fn get_mut(&mut self, row: usize, col: usize) -> Result<&mut T, IndexError> {
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
        I: IntoIterator<Item = usize>,
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
        I: IntoIterator<Item = usize>,
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
        I1: IntoIterator<Item = usize>,
        I2: IntoIterator<Item = usize>,
        I2::IntoIter: Clone,
    {
        let cols = cols.into_iter();

        rows.into_iter()
            .map(|row| {
                let row_ptr = self.0.get_mut(row).ok_or(IndexError::Row(row))? as *mut [T; C];

                cols.clone()
                    .map(|col| {
                        // Safety
                        // Enforced by caller
                        unsafe { &mut *row_ptr }
                            .get_mut(col)
                            .ok_or(IndexError::Column(col))
                    })
                    .collect::<Result<Vec<&mut T>, IndexError>>()
            })
            .collect::<Result<Vec<_>, IndexError>>()
    }
}
