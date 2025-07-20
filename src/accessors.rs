use crate::errors::IndexError;
use crate::Matrix;
use std::array;
use std::ops::{Index, IndexMut};

impl<T, const R: usize, const C: usize> Matrix<T, R, C> {
    /// Returns whether a matrix has the same number of rows as columns
    ///
    /// # Examples
    /// ```
    /// use matrixes::Matrix;
    ///
    /// let square = Matrix::<i32, 2, 2>::new();
    /// let rectangle = Matrix::<i32, 6, 9>::new();
    ///
    /// assert!(square.is_square());
    /// assert!(!rectangle.is_square());
    /// ```
    pub const fn is_square(&self) -> bool {
        R == C
    }

    /// Returns the number of elements in the matrix
    ///
    /// # Examples
    ///
    /// ```
    /// use matrixes::Matrix;
    ///
    /// let big = Matrix::<i32, 9, 13>::new();
    /// let small = Matrix::<i32, 2, 4>::new();
    ///
    /// assert_eq!(big.size(), 117);
    /// assert_eq!(small.size(), 8);
    /// ```
    ///
    /// ```
    /// use matrixes::Matrix;
    ///
    /// let thin = Matrix::<i32, 300, 0>::new();
    /// let short = Matrix::<i32, 0, 300>::new();
    /// let empty = Matrix::<i32, 0, 0>::new();
    ///
    /// assert_eq!(thin.size(), 0);
    /// assert_eq!(short.size(), 0);
    /// assert_eq!(empty.size(), 0);
    /// ```
    pub const fn size(&self) -> usize {
        R * C
    }
}

// get

impl<T, const R: usize, const C: usize> Matrix<T, R, C> {
    /// Returns a reference to an element or an error
    ///
    /// # Errors
    /// - `row` must index within bounds
    /// - `col` must index within bounds
    ///
    /// # Examples
    ///
    /// ```
    /// use matrixes::{
    ///     errors::IndexError,
    ///     Matrix
    /// };
    ///
    /// let m = Matrix::<i32, 3, 3>::new();
    ///
    /// assert_eq!(m.get(0, 2), Ok(&0));
    /// assert_eq!(m.get(5, 2), Err(IndexError::Row(5)));
    /// assert_eq!(m.get(0, 3), Err(IndexError::Column(3)));
    /// assert_eq!(m.get(4, 4), Err(IndexError::Both(4, 4)));
    /// ```
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

    /// Returns an array of references to the elements of a row or an error
    ///
    /// # Errors
    /// `row` must index within bounds
    ///
    /// # Examples
    ///
    /// ```
    /// use matrixes::{
    ///     errors::IndexError,
    ///     Matrix
    /// };
    ///
    /// let m = Matrix::<i32, 3, 3>::new();
    ///
    /// assert_eq!(m.get_row(1), Ok([&0, &0, &0]));
    /// assert_eq!(m.get_row(4), Err(IndexError::Row(4)));
    /// ```
    pub fn get_row(&self, row: usize) -> Result<[&T; C], IndexError> {
        if row >= R {
            return Err(IndexError::Row(row));
        }

        Ok(array::from_fn(|col| &self.0[row][col]))
    }

    /// Returns an array of references to the elements of a column or an error
    ///
    /// # Errors
    /// `col` must index within bounds
    ///
    /// # Examples
    ///
    /// ```
    /// use matrixes::{
    ///     errors::IndexError,
    ///     Matrix
    /// };
    ///
    /// let m = Matrix::<i32, 3, 3>::new();
    ///
    /// assert_eq!(m.get_col(1), Ok([&0, &0, &0]));
    /// assert_eq!(m.get_col(4), Err(IndexError::Column(4)));
    /// ```
    pub fn get_col(&self, col: usize) -> Result<[&T; R], IndexError> {
        if col >= C {
            return Err(IndexError::Column(col));
        }

        Ok(array::from_fn(|row| &self.0[row][col]))
    }

    /// Returns a Vec of arrays of references to elements of indexed rows or an error
    ///
    /// # Errors
    /// All elements of `rows` must index within bounds
    ///
    /// # Examples
    ///
    /// ```
    /// use matrixes::{
    ///     errors::IndexError,
    ///     Matrix
    /// };
    ///
    /// let m = Matrix::<i32, 5, 5>::new();
    ///
    /// assert_eq!(m.get_rows(1..=3), Ok(vec![[&0, &0, &0, &0, &0], [&0, &0, &0, &0, &0], [&0, &0, &0, &0, &0]]));
    /// assert_eq!(m.get_rows(vec![0, 2, 9, 6]), Err(IndexError::Row(9)));
    /// ```
    pub fn get_rows<I>(&self, rows: I) -> Result<Vec<[&T; C]>, IndexError>
    where
        I: IntoIterator<Item = usize>,
    {
        rows.into_iter().map(|row| self.get_row(row)).collect()
    }

    /// Returns a Vec of arrays of references to elements of indexed columns or an error
    ///
    /// # Errors
    /// All elements of `cols` must index within bounds
    ///
    /// # Examples
    ///
    /// ```
    /// use matrixes::{
    ///     errors::IndexError,
    ///     Matrix
    /// };
    ///
    /// let m = Matrix::<i32, 5, 5>::new();
    ///
    /// assert_eq!(m.get_cols(0..3), Ok(vec![[&0, &0, &0, &0, &0], [&0, &0, &0, &0, &0], [&0, &0, &0, &0, &0]]));
    /// assert_eq!(m.get_cols(vec![6, 0, 2, 9]), Err(IndexError::Column(6)));
    /// ```
    pub fn get_cols<I>(&self, cols: I) -> Result<Vec<[&T; R]>, IndexError>
    where
        I: IntoIterator<Item = usize>,
    {
        cols.into_iter().map(|col| self.get_col(col)).collect()
    }

    /// Returns a Vec of Vecs of references to elements of the indexed rows and columns or an error
    ///
    /// # Errors
    ///
    /// - All elements of `rows` must index within bounds
    /// - All elements of `cols` must index within bounds
    ///
    /// # Examples
    ///
    /// ```
    /// use matrixes::{
    ///     Matrix,
    ///     errors::IndexError
    /// };
    ///
    /// let m = Matrix::<i32, 5, 5>::new();
    ///
    /// assert_eq!(m.get_area(0..2, 1..3), Ok(vec![vec![&0, &0], vec![&0, &0]]));
    /// assert_eq!(m.get_area(3..6, vec![2, 2, 9]), Err(IndexError::Column(9)));
    /// ```
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
    /// Returns a reference to an element without doing bounds checks
    ///
    /// # Safety
    /// - `row` must index within bounds
    /// - `col` must index within bounds
    ///
    /// # Examples
    ///
    /// ```
    /// use matrixes::{
    ///     errors::IndexError,
    ///     Matrix
    /// };
    ///
    /// let m = Matrix::<i32, 3, 3>::new();
    ///
    /// assert_eq!(unsafe { m.get_unchecked(0, 2) }, &0);
    /// ```
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

    /// Returns an array of references to the elements of a row of the matrix without doing bounds checks
    ///
    /// # Safety
    ///
    /// - `row` must index within bounds
    ///
    /// # Examples
    ///
    /// ```
    /// use matrixes::Matrix;
    ///
    /// let m = Matrix::<i32, 3, 3>::new();
    ///
    /// assert_eq!(unsafe {m.get_row_unchecked(2) }, [&0, &0, &0]);
    /// ```
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

    /// Returns an array of references to the elements of a column of the matrix without doing bounds checks
    ///
    /// # Safety
    ///
    /// - `col` must index within bounds
    ///
    /// # Examples
    ///
    /// ```
    /// use matrixes::Matrix;
    ///
    /// let m = Matrix::<i32, 3, 3>::new();
    ///
    /// assert_eq!(unsafe { m.get_col_unchecked(1) }, [&0, &0, &0]);
    /// ```
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

    /// Returns a Vec of arrays of references to elements of indexed rows without doing bounds checks
    ///
    /// # Safety
    ///
    /// - all elements of `rows` must index within bounds
    ///
    /// # Examples
    ///
    /// ```
    /// use matrixes::Matrix;
    ///
    /// let m = Matrix::<i32, 3, 3>::new();
    ///
    /// assert_eq!(unsafe { m.get_rows_unchecked(vec![1, 0, 2]) }, vec![[&0, &0, &0], [&0, &0, &0], [&0, &0, &0]]);
    /// ```
    pub unsafe fn get_rows_unchecked<I>(&self, rows: I) -> Vec<[&T; C]>
    where
        I: IntoIterator<Item = usize>,
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

    /// Returns a Vec of arrays of elements of the indexed columns without doing bounds checks
    ///
    /// # Safety
    ///
    /// - all elements of `cols` must index within bounds
    ///
    /// # Examples
    ///
    /// ```
    /// use matrixes::Matrix;
    ///
    /// let m = Matrix::<i32, 3, 3>::new();
    ///
    /// assert_eq!( unsafe { m.get_cols_unchecked(1..=2) }, vec![[&0, &0, &0], [&0, &0, &0]]);
    /// ```
    pub unsafe fn get_cols_unchecked<I>(&self, cols: I) -> Vec<[&T; R]>
    where
        I: IntoIterator<Item = usize>,
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

    /// Returns a Vec of Vecs of references to elements of the indexed rows and columns without doing bounds checks
    ///
    /// # Safety
    ///
    /// - all elements of `rows` must index within bounds
    /// - all elements of `cols` must index within bounds
    ///
    /// # Examples
    ///
    /// ```
    /// use matrixes::Matrix;
    ///
    /// let m = Matrix::<i32, 3, 3>::new();
    ///
    /// assert_eq!( unsafe { m.get_area_unchecked(vec![0, 2], vec![2, 0]) }, vec![vec![&0, &0], vec![&0, &0]]);
    /// ```
    pub unsafe fn get_area_unchecked<I1, I2>(&self, rows: I1, cols: I2) -> Vec<Vec<&T>>
    where
        I1: IntoIterator<Item = usize>,
        I2: IntoIterator<Item = usize>,
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
    /// Returns a reference to an element or an error
    ///
    /// # Errors
    /// - `row` must index within bounds
    /// - `col` must index within bounds
    ///
    /// # Examples
    ///
    /// ```
    /// use matrixes::{
    ///     errors::IndexError,
    ///     Matrix
    /// };
    ///
    /// let mut m = Matrix::<i32, 3, 3>::new();
    ///
    /// assert_eq!(m.get_mut(0, 2), Ok(&mut 0));
    /// assert_eq!(m.get_mut(5, 2), Err(IndexError::Row(5)));
    /// assert_eq!(m.get_mut(0, 3), Err(IndexError::Column(3)));
    /// assert_eq!(m.get_mut(4, 4), Err(IndexError::Both(4, 4)));
    /// ```
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

    /// Returns an array of references to the elements of a row or an error
    ///
    /// # Errors
    /// `row` must index within bounds
    ///
    /// # Examples
    ///
    /// ```
    /// use matrixes::{
    ///     errors::IndexError,
    ///     Matrix
    /// };
    ///
    /// let mut m = Matrix::<i32, 3, 3>::new();
    ///
    /// assert_eq!(m.get_mut_row(1), Ok([&mut 0, &mut 0, &mut 0]));
    /// assert_eq!(m.get_mut_row(4), Err(IndexError::Row(4)));
    /// ```
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

    /// Returns an array of references to the elements of a column or an error
    ///
    /// # Errors
    /// `col` must index within bounds
    ///
    /// # Examples
    ///
    /// ```
    /// use matrixes::{
    ///     errors::IndexError,
    ///     Matrix
    /// };
    ///
    /// let mut m = Matrix::<i32, 3, 3>::new();
    ///
    /// assert_eq!(m.get_mut_col(1), Ok([&mut 0, &mut 0, &mut 0]));
    /// assert_eq!(m.get_mut_col(4), Err(IndexError::Column(4)));
    /// ```
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

    /// Returns a Vec of arrays of references to elements of indexed rows or an error
    ///
    /// # Safety
    ///
    /// `rows` must have no duplicates
    ///
    /// # Errors
    ///
    /// All elements of `rows` must index within bounds
    ///
    /// # Examples
    ///
    /// ```
    /// use matrixes::{
    ///     errors::IndexError,
    ///     Matrix
    /// };
    ///
    /// let mut m = Matrix::<i32, 5, 5>::new();
    ///
    /// assert_eq!(unsafe { m.get_mut_rows(1..=3) }, Ok(vec![[&mut 0, &mut 0, &mut 0, &mut 0, &mut 0], [&mut 0, &mut 0, &mut 0, &mut 0, &mut 0], [&mut 0, &mut 0, &mut 0, &mut 0, &mut 0]]));
    /// assert_eq!(unsafe { m.get_mut_rows(vec![0, 2, 9, 6]) }, Err(IndexError::Row(9)));
    /// ```
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

    /// Returns a Vec of arrays of references to elements of indexed columns or an error
    ///
    /// # Safety
    ///
    /// - `cols` must have no duplicates
    ///
    /// # Errors
    ///
    /// All elements of `cols` must index within bounds
    ///
    /// # Examples
    ///
    /// ```
    /// use matrixes::{
    ///     errors::IndexError,
    ///     Matrix
    /// };
    ///
    /// let mut m = Matrix::<i32, 5, 5>::new();
    ///
    /// assert_eq!(unsafe { m.get_mut_cols(0..3) }, Ok(vec![[&mut 0, &mut 0, &mut 0, &mut 0, &mut 0], [&mut 0, &mut 0, &mut 0, &mut 0, &mut 0], [&mut 0, &mut 0, &mut 0, &mut 0, &mut 0]]));
    /// assert_eq!(unsafe { m.get_mut_cols(vec![6, 0, 2, 9]) }, Err(IndexError::Column(6)));
    /// ```
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

    /// Returns a Vec of Vecs of references to elements of the indexed rows and columns or an error
    ///
    /// # Safety
    ///
    /// - `rows` must have no duplicates
    /// - `cols` must have no duplicates
    ///
    /// # Errors
    ///
    /// - All elements of `rows` must index within bounds
    /// - All elements of `cols` must index within bounds
    ///
    /// # Examples
    ///
    /// ```
    /// use matrixes::{
    ///     errors::IndexError,
    ///     Matrix
    /// };
    ///
    /// let mut m = Matrix::<i32, 5, 5>::new();
    ///
    /// assert_eq!(unsafe { m.get_mut_area(0..2, 1..3) }, Ok(vec![vec![&mut 0, &mut 0], vec![&mut 0, &mut 0]]));
    /// assert_eq!(unsafe { m.get_mut_area(3..6, vec![2, 2, 9]) }, Err(IndexError::Column(9)));
    /// ```
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
    /// Returns a reference to an element without doing bounds checks
    ///
    /// # Safety
    /// - `row` must index within bounds
    /// - `col` must index within bounds
    ///
    /// # Examples
    ///
    /// ```
    /// use matrixes::{
    ///     errors::IndexError,
    ///     Matrix
    /// };
    ///
    /// let mut m = Matrix::<i32, 3, 3>::new();
    ///
    /// assert_eq!(unsafe { m.get_unchecked_mut(0, 2) }, &mut 0);
    /// ```
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

    /// Returns an array of references to the elements of a row of the matrix without doing bounds checks
    ///
    /// # Safety
    ///
    /// - `row` must index within bounds
    ///
    /// # Examples
    ///
    /// ```
    /// use matrixes::Matrix;
    ///
    /// let mut m = Matrix::<i32, 3, 3>::new();
    ///
    /// assert_eq!(unsafe {m.get_row_unchecked_mut(2) }, [&mut 0, &mut 0, &mut 0]);
    /// ```
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

    /// Returns an array of references to the elements of a column of the matrix without doing bounds checks
    ///
    /// # Safety
    ///
    /// - `col` must index within bounds
    ///
    /// # Examples
    ///
    /// ```
    /// use matrixes::Matrix;
    ///
    /// let mut m = Matrix::<i32, 3, 3>::new();
    ///
    /// assert_eq!(unsafe { m.get_col_unchecked_mut(1) }, [&mut 0, &mut 0, &mut 0]);
    /// ```
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

    /// Returns a Vec of arrays of references to elements of indexed rows without doing bounds checks
    ///
    /// # Safety
    ///
    /// - all elements of `rows` must index within bounds
    /// - `rows` must have no duplicates
    ///
    /// # Examples
    ///
    /// ```
    /// use matrixes::Matrix;
    ///
    /// let m = Matrix::<i32, 3, 3>::new();
    ///
    /// assert_eq!(unsafe { m.get_rows_unchecked(vec![1, 0, 2]) }, vec![[&0, &0, &0], [&0, &0, &0], [&0, &0, &0]]);
    /// ```
    pub unsafe fn get_rows_unchecked_mut<I>(&mut self, rows: I) -> Vec<[&mut T; C]>
    where
        I: IntoIterator<Item = usize>,
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

    /// Returns a Vec of arrays of elements of the indexed columns without doing bounds checks
    ///
    /// # Safety
    ///
    /// - all elements of `cols` must index within bounds
    /// - `cols` must have no duplicates
    ///
    /// # Examples
    ///
    /// ```
    /// use matrixes::Matrix;
    ///
    /// let mut m = Matrix::<i32, 3, 3>::new();
    ///
    /// assert_eq!( unsafe { m.get_cols_unchecked_mut(1..=2) }, vec![[&mut 0, &mut 0, &mut 0], [&mut 0, &mut 0, &mut 0]]);
    /// ```
    pub unsafe fn get_cols_unchecked_mut<I>(&mut self, cols: I) -> Vec<[&mut T; R]>
    where
        I: IntoIterator<Item = usize>,
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

    /// Returns a Vec of Vecs of references to elements of the indexed rows and columns without doing bounds checks
    ///
    /// # Safety
    ///
    /// - all elements of `rows` must index within bounds
    /// - `rows` must have no duplicates
    /// - all elements of `cols` must index within bounds
    /// - `cols` must have no duplicates
    ///
    /// # Examples
    ///
    /// ```
    /// use matrixes::Matrix;
    ///
    /// let mut m = Matrix::<i32, 3, 3>::new();
    ///
    /// assert_eq!( unsafe { m.get_area_unchecked_mut(vec![0, 2], vec![2, 0]) }, vec![vec![&mut 0, &mut 0], vec![&mut 0, &mut 0]]);
    /// ```
    pub unsafe fn get_area_unchecked_mut<I1, I2>(&mut self, rows: I1, cols: I2) -> Vec<Vec<&mut T>>
    where
        I1: IntoIterator<Item = usize>,
        I2: IntoIterator<Item = usize>,
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
