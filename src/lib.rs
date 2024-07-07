mod errors;

#[cfg(test)]
mod tests;

extern crate num;

pub use errors::{ IndexError, InversionError, MinorError, SizingError };
use num::{ One, Zero };
use std::{ fmt::Debug, ops::{ Add, AddAssign, Div, Index, IndexMut, Mul, Neg, Sub, SubAssign } };

#[derive(Clone, PartialEq, Eq)]
pub struct Matrix<T> where T: Copy {
    data: Vec<T>,
    rows: usize,
    columns: usize,
}

// constructors
impl<T> Matrix<T> where T: Copy {
    /// Creates a matrix of zero size
    pub fn empty() -> Self {
        return Matrix {
            data: vec![],
            rows: 0,
            columns: 0,
        };
    }

    /// Creates a new matrix with the specified rows and columns initialized to 0 or a sizing error.
    ///
    /// # Errors
    ///
    /// If one of rows or columns is zero, both must be zero.
    pub fn new(rows: usize, columns: usize) -> Result<Self, SizingError> where T: Zero {
        if rows == 0 && columns != 0 {
            return Err(SizingError::Row(0));
        }

        if columns == 0 && rows != 0 {
            return Err(SizingError::Column(0));
        }

        Ok(Self {
            data: vec![T::zero(); rows * columns],
            rows,
            columns,
        })
    }

    /// Returns an Option to a new identity matrix with dimensions n x n.
    ///
    /// An identity matrix is a square matrix where the elements of the leading diagonal have a value of one and all other elements have a value of zero.
    pub fn new_identity(n: usize) -> Self where T: Zero + One {
        Self {
            data: (0..n.pow(2))
                .map(|i| if i % (n + 1) == 0 { T::one() } else { T::zero() })
                .collect(),
            rows: n,
            columns: n,
        }
    }

    /// Creates a matrix with raw data of data, and columns of columns or a sizing error.
    ///
    /// # Errors
    ///
    /// data must have a length that is divisable by columns.
    pub fn new_with_data(columns: usize, data: Vec<T>) -> Result<Self, SizingError> {
        let len = data.len();

        if len == 0 && columns == 0 {
            return Ok(Matrix::empty());
        }

        if len == 0 {
            return Err(SizingError::Row(0));
        }

        if columns == 0 {
            return Err(SizingError::Column(0));
        }

        if len % columns != 0 {
            return Err(SizingError::Row(len % columns));
        }

        Ok(Self {
            data,
            rows: len / columns,
            columns,
        })
    }

    /// Creates a matrix from data, which must be a vec of rows of elements, or the index of the first row of an invalid length.
    ///
    /// # Errors
    ///
    /// If data has rows, rows must all be of the same, non-zero length.
    pub fn new_from_data(elements: &[Vec<T>]) -> Result<Self, usize> {
        let rows = elements.len();

        if rows == 0 {
            return Ok(Matrix::empty());
        }

        let columns = elements[0].len();

        if columns == 0 {
            return Err(0);
        }

        let mut data: Vec<T> = Vec::with_capacity(rows * columns);

        for row in 0..rows {
            if elements[row].len() != columns {
                return Err(row);
            }

            for e in &elements[row] {
                data.push(*e);
            }
        }

        Ok(Self {
            data,
            rows,
            columns,
        })
    }

    /// Returns a matrix with values defined by the closure or a SizingError.
    ///
    /// # Errors
    ///
    /// If one of rows or columns is zero, both must be zero.
    pub fn new_from_closure(
        f: impl Fn(usize, usize) -> T,
        rows: usize,
        columns: usize
    ) -> Result<Self, SizingError> {
        if rows == 0 && columns != 0 {
            return Err(SizingError::Row(0));
        }

        if columns == 0 && rows != 0 {
            return Err(SizingError::Column(0));
        }

        let data = (0..rows)
            .flat_map(|i| (0..columns).map(|j| f(i, j)).collect::<Vec<_>>())
            .collect();

        Ok(Self { data, rows, columns })
    }

    /// Returns a matrix that has specified value down the leading diagonal and is zero everywhere else or a SizingError.
    ///
    /// # Errors
    ///
    /// If one of rows or columns is zero, both must be zero.
    pub fn new_diagonal(value: T, rows: usize, columns: usize) -> Result<Self, SizingError>
        where T: Zero
    {
        Matrix::new_from_closure(
            |i, j| {
                if i == j { value } else { T::zero() }
            },
            rows,
            columns
        )
    }
}

// getters
impl<T> Matrix<T> where T: Copy {
    /// Returns the data as a shared slice.
    pub fn data(&self) -> &[T] {
        &self.data
    }

    /// Returns the number of rows.
    pub fn rows(&self) -> usize {
        self.rows
    }

    /// Returns the number of columns.
    pub fn columns(&self) -> usize {
        self.columns
    }

    /// Returns the number of items in the matrix.
    pub fn size(&self) -> usize {
        self.data.len()
    }

    /// Returns whether the matrix is square in shape.
    pub fn is_square(&self) -> bool {
        self.rows == self.columns && self.rows != 0
    }

    /// Returns a reference to the indexed element or an IndexError
    ///
    /// # Errors
    ///
    /// Must index an element that exists.
    pub fn get(&self, row: usize, column: usize) -> Result<&T, IndexError> {
        if row >= self.rows && column >= self.columns {
            return Err(IndexError::Both(row, column));
        }

        if row >= self.rows {
            return Err(IndexError::Row(row));
        }

        if column >= self.columns {
            return Err(IndexError::Column(column));
        }

        Ok(&self.data[row * self.columns + column])
    }

    /// Returns indexed row as an option to a vec of references.
    ///
    /// # Errors
    ///
    /// index must refer to a row that exists.
    pub fn get_row(&self, index: usize) -> Option<Vec<&T>> {
        if index >= self.rows {
            return None;
        }

        Some((0..self.columns).map(|c| &self[(index, c)]).collect())
    }

    /// Returns an option to a vec of the rows indexed by the iterator.
    ///
    /// # Error
    ///
    /// All elements of rows must validly index the matrix.
    pub fn get_rows(&self, rows: impl Iterator<Item = usize>) -> Option<Vec<Vec<&T>>> {
        rows.map(|r| self.get_row(r)).collect::<Option<Vec<_>>>()
    }

    /// Returns indexed column as an option to a vec of references.
    ///
    /// # Errors
    ///
    /// index must refer to a column that exists.
    pub fn get_column(&self, index: usize) -> Option<Vec<&T>> {
        if index >= self.columns {
            return None;
        }

        Some((0..self.rows).map(|r| &self[(r, index)]).collect())
    }

    /// Returns an option to a vec of the columns indexed by the iterator.
    ///
    /// # Error
    ///
    /// All elements of columns must validly index the matrix.
    pub fn get_columns(&self, columns: impl Iterator<Item = usize>) -> Option<Vec<Vec<&T>>> {
        columns.map(|c| self.get_column(c)).collect::<Option<Vec<Vec<&T>>>>()
    }
}

// mut getters
impl<T> Matrix<T> where T: Copy {
    /// Returns data as a mutable shared slice.
    pub fn data_mut(&mut self) -> &mut [T] {
        &mut self.data
    }

    /// Returns a mutable reference to the indexed element or an IndexError
    ///
    /// # Errors
    ///
    /// Must index an element that exists.
    pub fn get_mut(&mut self, row: usize, column: usize) -> Result<&mut T, IndexError> {
        if row >= self.rows && column >= self.columns {
            return Err(IndexError::Both(row, column));
        }

        if row >= self.rows {
            return Err(IndexError::Row(row));
        }

        if column >= self.columns {
            return Err(IndexError::Column(column));
        }

        Ok(&mut self.data[row * self.columns + column])
    }

    /// Returns indexed row as an option to a vec of mutable references.
    ///
    /// # Errors
    ///
    /// index must refer to a row that exists.
    pub fn get_mut_row(&mut self, index: usize) -> Option<Vec<&mut T>> {
        if index >= self.rows {
            return None;
        }

        Some(self.data[index * self.columns..(index + 1) * self.columns].iter_mut().collect())
    }

    /// Returns indexed column as an option to a vec of mutable references.
    ///
    /// # Errors
    ///
    /// index must refer to a column that exists.
    pub fn get_mut_column(&mut self, index: usize) -> Option<Vec<&mut T>> {
        if index >= self.columns {
            return None;
        }

        Some(self.data.iter_mut().skip(index).step_by(self.columns).collect())
    }
}

// swappers
impl<T> Matrix<T> where T: Copy + 'static {
    /// Swaps the indexed elements or returns an indexing error.
    ///
    /// # Errors
    ///
    /// el1 and el2 must refer to valid elements of the matrix
    pub fn swap_elements(
        &mut self,
        el1: (usize, usize),
        el2: (usize, usize)
    ) -> Result<(), IndexError> {
        if el1.0 >= self.rows && el1.1 >= self.columns {
            return Err(IndexError::Both(el1.0, el1.1));
        }

        if el1.0 >= self.rows {
            return Err(IndexError::Row(el1.0));
        }

        if el1.1 >= self.columns {
            return Err(IndexError::Column(el1.1));
        }

        if el2.0 >= self.rows && el2.1 >= self.columns {
            return Err(IndexError::Both(el2.0, el2.1));
        }

        if el2.0 >= self.rows {
            return Err(IndexError::Row(el2.0));
        }

        if el2.1 >= self.columns {
            return Err(IndexError::Column(el2.1));
        }

        let temp = self[el1];
        self[el1] = self[el2];
        self[el2] = temp;

        Ok(())
    }

    /// Swaps the indexed rows or returns the invalid row index.
    ///
    /// # Errors
    ///
    /// row1 and row2 must refer to rows that exist
    pub fn swap_rows(&mut self, row1: usize, row2: usize) -> Result<(), IndexError> {
        if row1 == row2 {
            return Ok(());
        }

        let first_copy = self
            .get_row(row1)
            .ok_or(IndexError::Row(row1))?
            .iter()
            .map(|&e| *e)
            .collect::<Vec<_>>();

        let second_copy = self
            .get_row(row2)
            .ok_or(IndexError::Row(row2))?
            .iter()
            .map(|&e| *e)
            .collect::<Vec<_>>();

        let cols = self.columns;

        let mut first = self.get_mut_row(row1).unwrap();

        for i in 0..cols {
            *first[i] = second_copy[i];
        }

        let mut second = self.get_mut_row(row2).unwrap();

        for i in 0..cols {
            *second[i] = first_copy[i];
        }

        Ok(())
    }

    /// Swaps the indexed columns or returns the invalid column index.
    ///
    /// # Errors
    ///
    /// col1 and col2 must refer to columns that exist.
    pub fn swap_columns(&mut self, col1: usize, col2: usize) -> Result<(), IndexError> {
        if col1 == col2 {
            return Ok(());
        }

        let first_copy = self
            .get_column(col1)
            .ok_or(IndexError::Column(col1))?
            .iter()
            .map(|&e| *e)
            .collect::<Vec<_>>();

        let second_copy = self
            .get_column(col2)
            .ok_or(IndexError::Column(col2))?
            .iter()
            .map(|&e| *e)
            .collect::<Vec<_>>();

        let rows = self.rows;

        let mut first = self.get_mut_column(col1).unwrap();

        for i in 0..rows {
            *first[i] = second_copy[i];
        }

        let mut second = self.get_mut_column(col2).unwrap();

        for i in 0..rows {
            *second[i] = first_copy[i];
        }

        Ok(())
    }
}

// operations
impl<T> Matrix<T> where T: Copy {
    /// Multiplies each element of the matrix by factor.
    pub fn scale(&mut self, factor: T) where T: Mul<Output = T> {
        for e in self.data.iter_mut() {
            *e = *e * factor;
        }
    }

    /// Multiplies each element of indexed row by factor or returns the invalid index.
    ///
    /// # Errors
    ///
    /// row must refer to a row that exists.
    pub fn scale_row(&mut self, row: usize, factor: T) -> Result<(), IndexError>
        where T: Mul<Output = T>
    {
        for t in self.get_mut_row(row).ok_or(IndexError::Row(row))? {
            *t = *t * factor;
        }

        Ok(())
    }

    /// Adds source row scaled by factor to target row or returns the invalid index.
    ///
    /// # Errors
    ///
    /// source and target must refer to rows that exist.
    pub fn add_scaled_row(
        &mut self,
        source: usize,
        target: usize,
        factor: T
    ) -> Result<(), IndexError>
        where T: Add<Output = T> + Mul<Output = T>
    {
        let source_copy = self
            .get_row(source)
            .ok_or(IndexError::Row(source))?
            .iter()
            .map(|&e| *e)
            .collect::<Vec<_>>();

        let cols = self.columns;

        let mut target = self.get_mut_row(target).ok_or(IndexError::Row(target))?;

        for i in 0..cols {
            *target[i] = *target[i] + source_copy[i] * factor;
        }

        Ok(())
    }

    /// Multiplies each element of indexed column by factor or returns the invalid index.
    ///
    /// # Errors
    ///
    /// column must refer to a column that exists.
    pub fn scale_column(&mut self, column: usize, factor: T) -> Result<(), IndexError>
        where T: Mul<Output = T>
    {
        for t in self.get_mut_column(column).ok_or(IndexError::Column(column))? {
            *t = *t * factor;
        }

        Ok(())
    }

    /// Adds source column scaled by factor to target column or returns the invalid index.
    ///
    /// # Errors
    ///
    /// source and target must refer to columns that exist.
    pub fn add_scaled_column(
        &mut self,
        source: usize,
        target: usize,
        factor: T
    ) -> Result<(), IndexError>
        where T: Add<Output = T> + Mul<Output = T>
    {
        let source_clone = self
            .get_column(source)
            .ok_or(IndexError::Column(source))?
            .iter()
            .map(|&e| *e)
            .collect::<Vec<_>>();

        let rows = self.rows;

        let mut target = self.get_mut_column(target).ok_or(IndexError::Column(target))?;

        for i in 0..rows {
            *target[i] = *target[i] + source_clone[i] * factor;
        }

        Ok(())
    }

    /// Edits the boundries of the matrix while maintaing capacity or returns an index error.
    ///
    /// # Errors
    ///
    /// bounds must have the same size as the matrix
    pub fn resize(&mut self, bounds: (usize, usize)) -> Result<(), SizingError> {
        if bounds.0 == 0 && bounds.1 != 0 {
            return Err(SizingError::Row(0));
        }

        if bounds.1 == 0 && bounds.0 != 0 {
            return Err(SizingError::Column(0));
        }

        if bounds.0 * bounds.1 != self.size() {
            return Err(SizingError::Both(bounds.0, bounds.1));
        }

        self.rows = bounds.0;
        self.columns = bounds.1;

        Ok(())
    }

    /// Removes the data of the selected row and changes to bounds to match or returns the invalid index.
    ///
    /// Errors
    ///
    /// row must refer to a row that exists
    pub fn remove_row(&mut self, row: usize) -> Result<(), IndexError> {
        if row >= self.rows {
            return Err(IndexError::Row(row));
        }

        self.data.drain(row * self.columns..(row + 1) * self.columns);
        self.rows -= 1;

        if self.rows == 0 {
            self.columns = 0;
        }

        Ok(())
    }

    /// Removes the data of the selected column and changes to bounds to match or returns the invalid index.
    ///
    /// Errors
    ///
    /// column must refer to a row that exists
    pub fn remove_column(&mut self, column: usize) -> Result<(), IndexError> {
        if column >= self.columns {
            return Err(IndexError::Column(column));
        }

        self.columns -= 1;
        for r in 0..self.rows {
            self.data.remove(r * self.columns + column);
        }

        if self.rows == 0 {
            self.columns = 0;
        }

        Ok(())
    }

    /// Adds a row with an index of row and values of data or returns the an index error.
    ///
    /// Errors
    ///
    /// row must refer to a row adjacent to a row that exists, data must have the same number of elements as there are columns.
    pub fn insert_row(&mut self, row: usize, data: &[T]) -> Result<(), SizingError> {
        let len = data.len();

        if row > self.rows && len != self.columns && self.columns != 0 {
            return Err(SizingError::Both(row, len));
        }

        if row > self.rows {
            return Err(SizingError::Row(row));
        }

        if len != self.columns && self.columns != 0 {
            return Err(SizingError::Column(len));
        }

        self.rows += 1;
        for (col, e) in data.iter().enumerate() {
            self.data.insert(row * self.columns + col, *e);
        }

        if self.columns == 0 {
            self.columns = data.len();
        }

        Ok(())
    }

    /// Adds a row with an index of row and values of data or returns an index error.
    ///
    /// Errors
    ///
    /// column must refer to a column adjacent to a row that exists, data must have the same number of elements as there are rows.
    pub fn insert_column(&mut self, column: usize, data: &[T]) -> Result<(), SizingError> {
        let len = data.len();

        if column > self.columns && len != self.rows && self.rows != 0 {
            return Err(SizingError::Both(len, column));
        }

        if column > self.columns {
            return Err(SizingError::Column(column));
        }

        if len != self.rows && self.rows != 0 {
            return Err(SizingError::Row(len));
        }

        self.columns += 1;
        for (row, e) in data.iter().enumerate() {
            self.data.insert(row * self.columns + column, *e);
        }

        if self.rows == 0 {
            self.rows = data.len();
        }

        Ok(())
    }

    /// Adds content of other into new rows below the existing data or returns the invalid size.
    ///
    /// Errors
    ///
    /// other must have the same number of columns as the matrix.
    pub fn join_matrix_below(&mut self, other: &Matrix<T>) -> Result<(), SizingError> {
        if other.columns != self.columns && self.columns != 0 {
            return Err(SizingError::Column(other.columns));
        }

        self.rows += other.rows;
        self.data.append(&mut other.data.clone());

        if self.columns == 0 {
            self.columns = other.columns;
        }

        Ok(())
    }

    /// Adds content of other into new rows above the existing data or returns the invalid size.
    ///
    /// Errors
    ///
    /// other must have the same number of columns as the matrix.
    pub fn join_matrix_above(&mut self, other: &Matrix<T>) -> Result<(), SizingError> {
        if other.columns != self.columns && self.columns != 0 {
            return Err(SizingError::Column(other.columns));
        }

        self.rows += other.rows;
        let mut clone = other.data.clone();
        clone.append(&mut self.data);
        self.data = clone;

        if self.columns == 0 {
            self.columns = other.columns;
        }

        Ok(())
    }

    /// Adds content of other into new columns to the left of the existing data or returns the invalid size.
    ///
    /// Errors
    ///
    /// other must have the same number of rows as the matrix.
    pub fn join_matrix_left(&mut self, other: &Matrix<T>) -> Result<(), SizingError> {
        if other.rows != self.rows && self.rows != 0 {
            return Err(SizingError::Row(other.rows));
        }

        self.columns += other.columns;
        for (row, chunk) in other.data.chunks(other.columns).enumerate() {
            for (col, el) in chunk.iter().enumerate() {
                self.data.insert(row * self.columns + col, *el);
            }
        }

        if self.rows == 0 {
            self.rows = other.rows;
        }

        Ok(())
    }

    /// Adds content of other into new columns to the right of the existing data or returns the invalid size.
    ///
    /// Errors
    ///
    /// other must have the same number of rows as the matrix.
    pub fn join_matrix_right(&mut self, other: &Matrix<T>) -> Result<(), SizingError> {
        if other.rows != self.rows && self.rows != 0 {
            return Err(SizingError::Row(other.rows));
        }

        for (row, chunk) in other.data.chunks(other.columns).enumerate() {
            let r = row + 1;
            for (col, el) in chunk.iter().enumerate() {
                self.data.insert(r * self.columns + row * other.columns + col, *el);
            }
        }
        self.columns += other.columns;

        if self.rows == 0 {
            self.rows = other.rows;
        }

        Ok(())
    }
}

// derivers
impl<T> Matrix<T> where T: Copy {
    /// Creates a transpose matrix, whose rows are equivalent to the base matrix's columns.
    pub fn transpose(&self) -> Self {
        let mut elements: Vec<T> = vec![];

        for c in self.get_columns(0..self.columns).unwrap() {
            elements.append(
                &mut c
                    .iter()
                    .map(|e| **e)
                    .collect()
            );
        }

        Matrix {
            data: elements,
            rows: self.columns,
            columns: self.rows,
        }
    }

    /// Returns the minor of the indexed element or a minor error.
    ///
    /// The minor is the determinant of the sub-matrix generated by removing the row and column of the indexed row.
    ///
    /// # Errors
    ///
    /// matrix must be square and indexed element must exist.
    pub fn minor(&self, element: (usize, usize)) -> Result<T, MinorError>
        where T: Mul<Output = T> + Sub<Output = T> + Zero
    {
        if !self.is_square() {
            return Err(MinorError::NotSquare);
        }

        if element.0 >= self.rows && element.1 >= self.columns {
            return Err(IndexError::Both(element.0, element.1).into());
        }

        if element.0 >= self.rows {
            return Err(IndexError::Row(element.0).into());
        }

        if element.1 >= self.columns {
            return Err(IndexError::Column(element.1).into());
        }

        let mut copy = self.clone();

        copy.remove_row(element.0)?;
        copy.remove_column(element.1)?;

        Ok(copy.determinant().unwrap())
    }

    /// Returns an option to a new matrix constructed of the minors of each element of the matrix.
    ///
    /// # Errors
    ///
    /// Matrix must be square.
    pub fn minor_matrix(&self) -> Option<Self> where T: Mul<Output = T> + Sub<Output = T> + Zero {
        if !self.is_square() {
            return None;
        }

        Matrix::new_with_data(
            self.columns,
            (0..self.size())
                .map(|n| { self.minor((n / self.columns, n % self.columns)).unwrap() })
                .collect()
        ).ok()
    }

    /// Returns an option to the matrix of minors with every other element negated.
    ///
    /// Matrix must be square.
    pub fn cofactor(&self) -> Option<Self>
        where T: Neg<Output = T> + Mul<Output = T> + Sub<Output = T> + Zero
    {
        let mut out = self.minor_matrix()?;

        for (n, e) in out.data.iter_mut().enumerate() {
            if (n / self.columns + (n % self.columns)) % 2 == 1 {
                *e = e.neg();
            }
        }

        Some(out)
    }

    /// Returns an option to the transpose of the cofactor of the matrix.
    ///
    /// # Errors
    ///
    /// Matrix must be square.
    pub fn adjunct(&self) -> Option<Self>
        where T: Neg<Output = T> + Mul<Output = T> + Sub<Output = T> + Zero
    {
        Some(self.cofactor()?.transpose())
    }

    /// Returns an option to the determinant of the matrix.
    ///
    /// # Errors
    ///
    /// Matrix must be square.
    pub fn determinant(&self) -> Option<T> where T: Mul<Output = T> + Zero + Sub<Output = T> {
        if !self.is_square() {
            return None;
        }

        if self.rows == 1 {
            return Some(self.data[0]);
        }

        Some(
            self
                .get_row(0)
                .unwrap()
                .iter()
                .enumerate()
                .fold(T::zero(), |res, (c, e)| {
                    let det = **e * self.minor((0, c)).unwrap();
                    if c % 2 == 0 {
                        res + det
                    } else {
                        res - det
                    }
                })
        )
    }

    /// Returns the adjunct scaled by the inverse of the determinant or an inversion error.
    ///
    /// # Errors
    ///
    /// Matrix must be square, determinant must not be zero.
    ///
    /// # Warning
    ///
    /// May give an incorrect result on types with strong rounding on division such as integers.
    pub fn inverse(&self) -> Result<Self, InversionError>
        where
            T: Sub<Output = T> +
                Mul<Output = T> +
                Div<Output = T> +
                Neg<Output = T> +
                Zero +
                One +
                PartialEq
    {
        let det = self.determinant().ok_or(InversionError::NotSquare)?;

        if det == T::zero() {
            return Err(InversionError::InvalidDeterminant);
        }

        let mut out = self.adjunct().unwrap();
        out.scale(T::one() / det);

        Ok(out)
    }

    /// Returns the adjunct scaled by the inverse of the derminant or an inversion error.
    ///
    /// # Errors
    ///
    /// Matrix must be square, determinant must not be zero.
    ///
    /// # Warning
    ///
    /// May give an incorrect result or unwarrented error on types with strong rounding on division such as integers.
    pub fn fast_inverse(&self) -> Result<Self, InversionError>
        where T: Copy + Zero + One + Div<Output = T> + Neg<Output = T> + PartialEq
    {
        if !self.is_square() {
            return Err(InversionError::NotSquare);
        }

        let mut clone = self.clone();
        let mut out = Matrix::new_identity(clone.rows);

        for c in 0..clone.columns {
            // set diagonal element to one
            if !T::is_one(&clone[(c, c)]) {
                if T::is_zero(&clone[(c, c)]) {
                    return Err(InversionError::InvalidDeterminant);
                }
                let factor = T::one() / clone[(c, c)];
                clone.scale_row(c, factor).unwrap();
                out.scale_row(c, factor).unwrap();
            }

            // set elements above diagonal to zero
            for r in 0..c {
                if !T::is_zero(&clone[(r, c)]) {
                    let factor = clone[(r, c)].neg();
                    clone.add_scaled_row(c, r, factor).unwrap();
                    out.add_scaled_row(c, r, factor).unwrap();
                }
            }

            // set elements below diagonal to zero
            for r in c + 1..clone.rows {
                if !T::is_zero(&clone[(r, c)]) {
                    let factor = clone[(r, c)].neg();
                    clone.add_scaled_row(c, r, factor).unwrap();
                    out.add_scaled_row(c, r, factor).unwrap();
                }
            }
        }

        Ok(out)
    }

    /// Returns with edited boundries while maintaining capacity or returns an index error.
    ///
    /// Errors
    ///
    /// Matrix of size bounds must fit the same amount of data as this.
    pub fn as_resize(&self, bounds: (usize, usize)) -> Result<Matrix<T>, SizingError> {
        if bounds.0 == 0 {
            return Err(SizingError::Row(0));
        }

        if bounds.1 == 0 {
            return Err(SizingError::Column(0));
        }

        if bounds.0 * bounds.1 != self.size() {
            return Err(SizingError::Both(bounds.0, bounds.1));
        }

        Ok(Matrix {
            data: self.data.clone(),
            rows: bounds.0,
            columns: bounds.1,
        })
    }
}

impl<T> Debug for Matrix<T> where T: Copy + Debug {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.rows <= 1 {
            return write!(f, "{:?}", self.data);
        }

        writeln!(f, "{:?}", self.get_row(0).unwrap())?;
        let middle = self.get_rows(1..self.rows - 1).unwrap();
        for row in middle {
            writeln!(f, "{:?}", row)?;
        }
        write!(f, "{:?}", self.get_row(self.rows - 1).unwrap())
    }
}

impl<T> Index<(usize, usize)> for Matrix<T> where T: Copy {
    type Output = T;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        assert!(
            index.0 < self.rows && index.1 < self.columns,
            "Index out of bounds. index: {:?}, matrix bounds: {:?}",
            index,
            (self.rows, self.columns)
        );

        &self.data[index.0 * self.columns + index.1]
    }
}

impl<T> IndexMut<(usize, usize)> for Matrix<T> where T: Copy {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        assert!(
            index.0 < self.rows && index.1 < self.columns,
            "Index out of bounds. index: {:?}, matrix bounds: {:?}",
            index,
            (self.rows, self.columns)
        );

        &mut self.data[index.0 * self.columns + index.1]
    }
}

impl<T> AddAssign for Matrix<T> where T: Add<Output = T> + Copy {
    /// # Panics
    ///
    /// rhs must have the same rows and columns as the matrix.
    fn add_assign(&mut self, rhs: Self) {
        assert_eq!(self.rows, rhs.rows, "Mismatched rows.");
        assert_eq!(self.columns, rhs.columns, "Mismatched columns");

        for (s, o) in self.data.iter_mut().zip(rhs.data.iter()) {
            *s = *s + *o;
        }
    }
}

impl<T, U> Add for Matrix<T> where T: Add<Output = U> + Copy, U: Copy {
    type Output = Result<Matrix<U>, SizingError>;

    fn add(self, rhs: Self) -> Self::Output {
        if self.rows != rhs.rows && self.columns != rhs.columns {
            return Err(SizingError::Both(rhs.rows, rhs.columns));
        }

        if self.rows != rhs.rows {
            return Err(SizingError::Row(rhs.rows));
        }

        if self.columns != rhs.columns {
            return Err(SizingError::Column(rhs.columns));
        }

        Ok(Matrix {
            data: self.data
                .iter()
                .zip(rhs.data.iter())
                .map(|(s, r)| *s + *r)
                .collect(),
            rows: self.rows,
            columns: self.columns,
        })
    }
}

impl<T, U> Add for &Matrix<T> where T: Add<Output = U> + Copy, U: Copy {
    type Output = Result<Matrix<U>, SizingError>;

    fn add(self, rhs: Self) -> Self::Output {
        if self.rows != rhs.rows && self.columns != rhs.columns {
            return Err(SizingError::Both(rhs.rows, rhs.columns));
        }

        if self.rows != rhs.rows {
            return Err(SizingError::Row(rhs.rows));
        }

        if self.columns != rhs.columns {
            return Err(SizingError::Column(rhs.columns));
        }

        Ok(Matrix {
            data: self.data
                .iter()
                .zip(rhs.data.iter())
                .map(|(s, r)| *s + *r)
                .collect(),
            rows: self.rows,
            columns: self.columns,
        })
    }
}

impl<T> SubAssign for Matrix<T> where T: Sub<Output = T> + Copy {
    fn sub_assign(&mut self, rhs: Self) {
        assert!(self.rows == rhs.rows, "Mismatched rows");
        assert!(self.columns == rhs.columns, "Mismatched columns");

        for (s, o) in self.data.iter_mut().zip(rhs.data.iter()) {
            *s = *s - *o;
        }
    }
}

impl<T, U> Sub for Matrix<T> where T: Sub<Output = U> + Copy, U: Copy {
    type Output = Result<Matrix<U>, SizingError>;

    fn sub(self, rhs: Self) -> Self::Output {
        if self.rows != rhs.rows && self.columns != rhs.columns {
            return Err(SizingError::Both(rhs.rows, rhs.columns));
        }

        if self.rows != rhs.rows {
            return Err(SizingError::Row(rhs.rows));
        }

        if self.columns != rhs.columns {
            return Err(SizingError::Column(rhs.columns));
        }

        Ok(Matrix {
            data: self.data
                .iter()
                .zip(rhs.data.iter())
                .map(|(s, r)| *s - *r)
                .collect(),
            rows: self.rows,
            columns: self.columns,
        })
    }
}

impl<T, U> Sub for &Matrix<T> where T: Sub<Output = U> + Copy, U: Copy {
    type Output = Result<Matrix<U>, SizingError>;
    fn sub(self, rhs: Self) -> Self::Output {
        if self.rows != rhs.rows && self.columns != rhs.columns {
            return Err(SizingError::Both(rhs.rows, rhs.columns));
        }

        if self.rows != rhs.rows {
            return Err(SizingError::Row(rhs.rows));
        }

        if self.columns != rhs.columns {
            return Err(SizingError::Column(rhs.columns));
        }

        Ok(Matrix {
            data: self.data
                .iter()
                .zip(rhs.data.iter())
                .map(|(s, r)| *s - *r)
                .collect(),
            rows: self.rows,
            columns: self.columns,
        })
    }
}

impl<T, U> Mul for Matrix<T> where T: Copy + Mul<Output = U>, U: Copy + Zero {
    type Output = Result<Matrix<U>, usize>;

    fn mul(self, rhs: Self) -> Self::Output {
        if self.columns != rhs.rows {
            return Err(rhs.rows);
        }

        Matrix::new_from_data(
            &(0..self.rows)
                .map(|r| {
                    (0..rhs.columns)
                        .map(|c| {
                            self.get_row(r)
                                .unwrap()
                                .iter()
                                .zip(rhs.get_column(c).unwrap().iter())
                                .fold(U::zero(), |res, (&el, &er)| res + *el * *er)
                        })
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>()
        )
    }
}

impl<T, U> Mul for &Matrix<T> where T: Copy + Mul<Output = U>, U: Copy + Zero {
    type Output = Result<Matrix<U>, usize>;

    fn mul(self, rhs: Self) -> Self::Output {
        if self.columns != rhs.rows {
            return Err(rhs.rows);
        }

        Matrix::new_from_data(
            &(0..self.rows)
                .map(|r| {
                    (0..rhs.columns)
                        .map(|c| {
                            self.get_row(r)
                                .unwrap()
                                .iter()
                                .zip(rhs.get_column(c).unwrap().iter())
                                .fold(U::zero(), |res, (&el, &er)| res + *el * *er)
                        })
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>()
        )
    }
}

impl<T, U> Neg for Matrix<T> where T: Neg<Output = U> + Copy, U: Copy {
    type Output = Matrix<U>;

    fn neg(self) -> Self::Output {
        Matrix {
            data: self.data
                .iter()
                .map(|t| t.neg())
                .collect(),
            rows: self.rows,
            columns: self.columns,
        }
    }
}

impl<T, U> Neg for &Matrix<T> where T: Neg<Output = U> + Copy, U: Copy {
    type Output = Matrix<U>;

    fn neg(self) -> Self::Output {
        Matrix {
            data: self.data
                .iter()
                .map(|t| t.neg())
                .collect(),
            rows: self.rows,
            columns: self.columns,
        }
    }
}

impl<T> Default for Matrix<T> where T: Copy {
    fn default() -> Self {
        Self {
            data: vec![],
            rows: 0,
            columns: 0,
        }
    }
}
