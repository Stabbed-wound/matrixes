use fixtures::*;
use matrixes::{errors::IndexError, Matrix};
use rstest::*;
use rstest_reuse::*;
use std::fmt::Debug;
use std::ops::Index;
use std::ops::IndexMut;

mod fixtures {
    use super::*;

    #[fixture]
    pub fn small_identity() -> Matrix<i32, 3, 3> {
        Matrix::new_identity()
    }

    #[fixture]
    pub fn four_by_three() -> Matrix<i32, 4, 3> {
        Matrix::from([[1, 2, 3], [9, 8, 7], [4, 5, 6], [12, 0, 4]])
    }

    #[fixture]
    pub fn one_by_five() -> Matrix<u8, 1, 5> {
        Matrix::from([[1, 4, 2, 5, 3]])
    }

    #[fixture]
    pub fn empty() -> Matrix<u128, 0, 0> {
        Matrix::from([])
    }

    #[fixture]
    pub fn no_rows() -> Matrix<i16, 0, 200> {
        Matrix::from([])
    }

    #[fixture]
    pub fn no_cols() -> Matrix<u32, 17, 0> {
        Matrix::from([
            [],
            [],
            [],
            [],
            [],
            [],
            [],
            [],
            [],
            [],
            [],
            [],
            [],
            [],
            [],
            [],
            [],
        ])
    }
}

#[rstest]
#[case(small_identity(), true)]
#[case(four_by_three(), false)]
#[case(one_by_five(), false)]
#[case(empty(), true)]
#[case(no_rows(), false)]
#[case(no_cols(), false)]
fn is_square<T, const R: usize, const C: usize>(
    #[case] matrix: Matrix<T, R, C>,
    #[case] expected: bool,
) {
    assert_eq!(matrix.is_square(), expected);
}

#[rstest]
#[case(small_identity(), 9)]
#[case(four_by_three(), 12)]
#[case(one_by_five(), 5)]
#[case(empty(), 0)]
#[case(no_rows(), 0)]
#[case(no_cols(), 0)]
fn size<T, const R: usize, const C: usize>(
    #[case] matrix: Matrix<T, R, C>,
    #[case] expected: usize,
) {
    assert_eq!(matrix.size(), expected);
}

mod getters {
    use super::*;

    #[template]
    #[rstest]
    #[case(small_identity(), (0, 0), Ok(1))]
    #[case(small_identity(), (12, 2), Err(IndexError::Row(12)))]
    #[case(four_by_three(), (2, 1), Ok(5))]
    #[case(four_by_three(), (1, 31), Err(IndexError::Column(31)))]
    #[case(one_by_five(), (0, 4), Ok(3))]
    #[case(one_by_five(), (1, 200), Err(IndexError::Both(1, 200)))]
    #[case(empty(), (0, 0), Err(IndexError::Both(0, 0)))]
    #[case(no_rows(), (0, 0), Err(IndexError::Row(0)))]
    #[case(no_cols(), (0, 0), Err(IndexError::Column(0)))]
    fn getter<T, const R: usize, const C: usize>(
        #[case] matrix: Matrix<T, R, C>,
        #[case] (row, col): (usize, usize),
        #[case] expected: Result<T, IndexError>,
    ) {
    }

    #[apply(getter)]
    fn get<T, const R: usize, const C: usize>(
        #[case] matrix: Matrix<T, R, C>,
        #[case] (row, col): (usize, usize),
        #[case] expected: Result<T, IndexError>,
    ) where
        T: PartialEq + Debug + Copy,
    {
        assert_eq!(matrix.get(row, col).copied(), expected);
    }

    #[apply(getter)]
    fn get_mut<T, const R: usize, const C: usize>(
        #[case] mut matrix: Matrix<T, R, C>,
        #[case] (row, col): (usize, usize),
        #[case] expected: Result<T, IndexError>,
    ) where
        T: PartialEq + Debug + Copy,
    {
        assert_eq!(matrix.get_mut(row, col).copied(), expected);
    }
}

mod row_getters {
    use super::*;

    #[template]
    #[rstest]
    #[case(small_identity(), 1, Ok([0, 1, 0]))]
    #[case(small_identity(), 4, Err(IndexError::Row(4)))]
    #[case(four_by_three(), 0, Ok([1, 2, 3]))]
    #[case(four_by_three(), 19, Err(IndexError::Row(19)))]
    #[case(one_by_five(), 0, Ok([1, 4, 2, 5, 3]))]
    #[case(one_by_five(), 1, Err(IndexError::Row(1)))]
    #[case(empty(), 0, Err(IndexError::Row(0)))]
    #[case(no_rows(), 0, Err(IndexError::Row(0)))]
    #[case(no_cols(), 15, Ok([]))]
    fn row_getter<T, const R: usize, const C: usize>(
        #[case] matrix: Matrix<T, R, C>,
        #[case] row: usize,
        #[case] expected: Result<[T; C], IndexError>,
    ) {
    }

    #[apply(row_getter)]
    fn get_row<T, const R: usize, const C: usize>(
        #[case] matrix: Matrix<T, R, C>,
        #[case] row: usize,
        #[case] expected: Result<[T; C], IndexError>,
    ) where
        T: PartialEq + Debug + Copy,
    {
        assert_eq!(
            matrix.get_row(row).map(|row| row.map(|elem| *elem)),
            expected
        );
    }

    #[apply(row_getter)]
    fn get_mut_row<T, const R: usize, const C: usize>(
        #[case] mut matrix: Matrix<T, R, C>,
        #[case] row: usize,
        #[case] expected: Result<[T; C], IndexError>,
    ) where
        T: PartialEq + Debug + Copy,
    {
        assert_eq!(
            matrix.get_mut_row(row).map(|row| row.map(|elem| *elem)),
            expected
        );
    }
}

mod col_getters {
    use super::*;

    #[template]
    #[rstest]
    #[case(small_identity(), 0, Ok([1, 0, 0]))]
    #[case(small_identity(), 4, Err(IndexError::Column(4)))]
    #[case(four_by_three(), 0, Ok([1, 9, 4, 12]))]
    #[case(four_by_three(), 19, Err(IndexError::Column(19)))]
    #[case(one_by_five(), 3, Ok([5]))]
    #[case(one_by_five(), 5, Err(IndexError::Column(5)))]
    #[case(empty(), 0, Err(IndexError::Column(0)))]
    #[case(no_rows(), 100, Ok([]))]
    #[case(no_cols(), 0, Err(IndexError::Column(0)))]
    fn col_getter<T, const R: usize, const C: usize>(
        #[case] matrix: Matrix<T, R, C>,
        #[case] row: usize,
        #[case] expected: Result<[T; R], IndexError>,
    ) {
    }

    #[apply(col_getter)]
    fn get_col<T, const R: usize, const C: usize>(
        #[case] matrix: Matrix<T, R, C>,
        #[case] row: usize,
        #[case] expected: Result<[T; R], IndexError>,
    ) where
        T: PartialEq + Debug + Copy,
    {
        assert_eq!(
            matrix.get_col(row).map(|col| col.map(|elem| *elem)),
            expected
        );
    }

    #[apply(col_getter)]
    fn get_mut_col<T, const R: usize, const C: usize>(
        #[case] mut matrix: Matrix<T, R, C>,
        #[case] row: usize,
        #[case] expected: Result<[T; R], IndexError>,
    ) where
        T: PartialEq + Debug + Copy,
    {
        assert_eq!(
            matrix.get_mut_col(row).map(|col| col.map(|elem| *elem)),
            expected
        );
    }
}

mod rows_getters {
    use super::*;

    #[template]
    #[rstest]
    #[case(small_identity(), 0..1, Ok(vec![[1, 0, 0]]))]
    #[case(small_identity(), 0..=3, Err(IndexError::Row(3)))]
    #[case(four_by_three(), vec![1, 3, 0], Ok(vec![[9, 8, 7], [12, 0, 4], [1, 2, 3]]))]
    #[case(one_by_five(), 500..500, Ok(vec![]))]
    #[case(one_by_five(), 12..14, Err(IndexError::Row(12)))]
    #[case(empty(), 0..1, Err(IndexError::Row(0)))]
    #[case(no_rows(), 0..1, Err(IndexError::Row(0)))]
    #[case(no_cols(), 0..1, Ok(vec![[]]))]
    fn rows_getter<T, I, const R: usize, const C: usize>(
        #[case] matrix: Matrix<T, R, C>,
        #[case] rows: I,
        #[case] expected: Result<Vec<[&T; C]>, IndexError>,
    ) {
    }

    #[apply(rows_getter)]
    fn get_rows<T, I, const R: usize, const C: usize>(
        #[case] matrix: Matrix<T, R, C>,
        #[case] rows: I,
        #[case] expected: Result<Vec<[T; C]>, IndexError>,
    ) where
        T: PartialEq + Debug + Copy,
        I: IntoIterator<Item = usize>,
    {
        assert_eq!(
            matrix
                .get_rows(rows)
                .map(|rows| { rows.into_iter().map(|row| row.map(|elem| *elem)).collect() }),
            expected
        );
    }

    #[apply(rows_getter)]
    fn get_mut_rows<T, I, const R: usize, const C: usize>(
        #[case] mut matrix: Matrix<T, R, C>,
        #[case] rows: I,
        #[case] expected: Result<Vec<[T; C]>, IndexError>,
    ) where
        T: PartialEq + Debug + Copy,
        I: IntoIterator<Item = usize>,
    {
        unsafe {
            assert_eq!(
                matrix
                    .get_mut_rows(rows)
                    .map(|rows| rows.into_iter().map(|row| row.map(|elem| *elem)).collect()),
                expected
            );
        }
    }
}

mod cols_getters {
    use super::*;

    #[template]
    #[rstest]
    #[case(small_identity(), 1..=1, Ok(vec![[0, 1, 0]]))]
    #[case(small_identity(), 0..=3, Err(IndexError::Column(3)))]
    #[case(four_by_three(), vec![1, 0], Ok(vec![[2, 8, 5, 0], [1, 9, 4, 12]]))]
    #[case(one_by_five(), 500..500, Ok(vec![]))]
    #[case(one_by_five(), 12..14, Err(IndexError::Column(12)))]
    #[case(empty(), 0..1, Err(IndexError::Column(0)))]
    #[case(no_rows(), 0..1, Ok(vec![[]]))]
    #[case(no_cols(), 0..1, Err(IndexError::Column(0)))]
    fn cols_getter<T, I, const R: usize, const C: usize>(
        #[case] matrix: Matrix<T, R, C>,
        #[case] cols: I,
        #[case] expected: Result<Vec<[T; R]>, IndexError>,
    ) {
    }

    #[apply(cols_getter)]
    fn get_cols<T, I, const R: usize, const C: usize>(
        #[case] matrix: Matrix<T, R, C>,
        #[case] cols: I,
        #[case] expected: Result<Vec<[T; R]>, IndexError>,
    ) where
        T: PartialEq + Debug + Copy,
        I: IntoIterator<Item = usize>,
    {
        assert_eq!(
            matrix
                .get_cols(cols)
                .map(|cols| cols.into_iter().map(|col| col.map(|elem| *elem)).collect()),
            expected
        );
    }

    #[apply(cols_getter)]
    fn get_mut_cols<T, I, const R: usize, const C: usize>(
        #[case] mut matrix: Matrix<T, R, C>,
        #[case] cols: I,
        #[case] expected: Result<Vec<[T; R]>, IndexError>,
    ) where
        T: PartialEq + Debug + Copy,
        I: IntoIterator<Item = usize>,
    {
        unsafe {
            assert_eq!(
                matrix
                    .get_mut_cols(cols)
                    .map(|cols| cols.into_iter().map(|col| col.map(|elem| *elem)).collect()),
                expected
            );
        }
    }
}

mod area_getters {
    use super::*;

    #[template]
    #[rstest]
    #[case(small_identity(), 0..2, 1..3, Ok(vec![vec![0, 0], vec![1, 0]]))]
    #[case(small_identity(), 0..4, 1..=1, Err(IndexError::Row(3)))]
    #[case(four_by_three(), 2..=2, 2..=2, Ok(vec![vec![6]]))]
    #[case(four_by_three(), 3..=3, 3..=3, Err(IndexError::Column(3)))]
    #[case(one_by_five(), 3..3, 1..4, Ok(vec![]))]
    #[case(one_by_five(), 5..=5, 5..=5, Err(IndexError::Both(5, 5)))]
    #[case(empty(), 0..1, 0..1, Err(IndexError::Both(0, 0)))]
    #[case(no_rows(), 0..1, 0..1, Err(IndexError::Row(0)))]
    #[case(no_cols(), 0..1, 0..1, Err(IndexError::Column(0)))]
    fn area_getter<T, I1, I2, const R: usize, const C: usize>(
        #[case] matrix: Matrix<T, R, C>,
        #[case] rows: I1,
        #[case] cols: I2,
        #[case] expected: Result<Vec<Vec<T>>, IndexError>,
    ) {
    }

    #[apply(area_getter)]
    fn get_area<T, I1, I2, const R: usize, const C: usize>(
        #[case] matrix: Matrix<T, R, C>,
        #[case] rows: I1,
        #[case] cols: I2,
        #[case] expected: Result<Vec<Vec<T>>, IndexError>,
    ) where
        T: PartialEq + Debug + Copy,
        I1: IntoIterator<Item = usize>,
        I2: IntoIterator<Item = usize>,
        I2::IntoIter: Clone,
    {
        assert_eq!(
            matrix.get_area(rows, cols).map(|rows| rows
                .into_iter()
                .map(|row| row.into_iter().copied().collect())
                .collect()),
            expected
        );
    }

    #[apply(area_getter)]
    fn get_mut_area<T, I1, I2, const R: usize, const C: usize>(
        #[case] mut matrix: Matrix<T, R, C>,
        #[case] rows: I1,
        #[case] cols: I2,
        #[case] expected: Result<Vec<Vec<T>>, IndexError>,
    ) where
        T: PartialEq + Debug + Copy,
        I1: IntoIterator<Item = usize>,
        I2: IntoIterator<Item = usize>,
        I2::IntoIter: Clone,
    {
        unsafe {
            assert_eq!(
                matrix.get_mut_area(rows, cols).map(|rows| {
                    rows.into_iter()
                        .map(|row| row.into_iter().map(|elem| *elem).collect())
                        .collect()
                }),
                expected
            );
        }
    }
}

#[rstest]
#[case(small_identity(), (0, 0), 1)]
#[case(four_by_three(), (2, 1), 5)]
#[case(one_by_five(), (0, 4), 3)]
fn indexing<T, const R: usize, const C: usize>(
    #[case] mut matrix: Matrix<T, R, C>,
    #[case] index: (usize, usize),
    #[case] expected: T,
) where
    T: PartialEq + Debug + Copy,
{
    assert_eq!(*Matrix::index(&matrix, index), expected);
    assert_eq!(*Matrix::index_mut(&mut matrix, index), expected);
}
