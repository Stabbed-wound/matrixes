use super::*;

#[test]
fn empty() {
    assert_eq!(Matrix::<i16>::empty(), Matrix {
        data: vec![],
        rows: 0,
        columns: 0,
    });
}

mod new {
    use super::*;

    #[test]
    fn handles_errors() {
        assert_eq!(Matrix::<u8>::new(0, 5), Err(SizingError::Row(0)));
        assert_eq!(Matrix::<i32>::new(2, 0), Err(SizingError::Column(0)));
    }

    #[test]
    fn creates_matrix() {
        assert_eq!(
            Matrix::<i8>::new(2, 5),
            Ok(Matrix {
                data: vec![0; 10],
                rows: 2,
                columns: 5,
            })
        );
        assert_eq!(
            Matrix::new(8, 3),
            Ok(Matrix {
                data: vec![0; 24],
                rows: 8,
                columns: 3,
            })
        );
        assert_eq!(
            Matrix::new(4, 4),
            Ok(Matrix {
                data: vec![0; 16],
                rows: 4,
                columns: 4,
            })
        );
    }
}

#[test]
fn new_identity() {
    assert_eq!(Matrix::new_identity(4), Matrix {
        data: vec![1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1],
        rows: 4,
        columns: 4,
    });
}

mod new_with_data {
    use super::*;

    #[test]
    fn handles_errors() {
        assert_eq!(Matrix::new_with_data(0, vec![1, 2, 3, 4]), Err(SizingError::Column(0)));
        assert_eq!(Matrix::<u8>::new_with_data(4, vec![]), Err(SizingError::Row(0)));
        assert_eq!(Matrix::new_with_data(3, vec![5; 7]), Err(SizingError::Row(1)));
    }

    #[test]
    fn creates_matrix() {
        let m = Matrix::new_with_data(7, (0u32..35).collect());

        assert_eq!(
            m,
            Ok(Matrix {
                data: (0u32..35).collect(),
                rows: 5,
                columns: 7,
            })
        )
    }
}

mod new_from_data {
    use super::*;

    #[test]
    fn handles_errors() {
        assert_eq!(Matrix::new_from_data(&vec![vec![], vec![1, 5, 6], vec![2, 6, 9]]), Err(0));
        assert_eq!(
            Matrix::new_from_data(
                &vec![
                    vec![1, 5, 3, 2, 7],
                    vec![1, 2, 45, 7, 3],
                    vec![65, 8, 5, 23, 67],
                    vec![123, 5, 47]
                ]
            ),
            Err(3)
        )
    }

    #[test]
    fn creates_matrix() {
        assert_eq!(
            Matrix::new_from_data(
                &vec![vec![1, 2, 3, 4], vec![2, 3, 4, 1], vec![3, 4, 1, 2], vec![4, 1, 2, 3]]
            ),
            Ok(Matrix {
                data: vec![1, 2, 3, 4, 2, 3, 4, 1, 3, 4, 1, 2, 4, 1, 2, 3],
                rows: 4,
                columns: 4,
            })
        );
        assert_eq!(
            Matrix::new_from_data(&vec![vec![4, 2, 1, 5, 3], vec![1, 2, 3, 4, 5]]),
            Ok(Matrix {
                data: vec![4, 2, 1, 5, 3, 1, 2, 3, 4, 5],
                rows: 2,
                columns: 5,
            })
        );
        assert_eq!(
            Matrix::<&str>::new_from_data(&vec![]),
            Ok(Matrix {
                data: vec![],
                rows: 0,
                columns: 0,
            })
        );
    }
}

mod new_from_closure {
    use super::*;

    #[test]
    fn handles_errors() {
        assert_eq!(
            Matrix::new_from_closure(|i, _j| i, 0, 6),
            Err(SizingError::Row(0))
        );
        assert_eq!(
            Matrix::new_from_closure(|_i, j| j, 13, 0),
            Err(SizingError::Column(0))
        );
    }

    #[test]
    fn creates_matrix() {
        assert_eq!(
            Matrix::new_from_closure(|i, j| i + j, 4, 6),
            Ok(Matrix {
                data: vec![0, 1, 2, 3, 4, 5, 1, 2, 3, 4, 5, 6, 2, 3, 4, 5, 6, 7, 3, 4, 5, 6, 7, 8],
                rows: 4,
                columns: 6,
            })
        );
        assert_eq!(
            Matrix::new_from_closure(|i, j| i * j, 0, 0),
            Ok(Matrix::empty())
        );
    }
}

mod new_diagonal {
    use super::*;

    #[test]
    fn handles_errors() {
        assert_eq!(Matrix::new_diagonal(7f32, 0, 5), Err(SizingError::Row(0)));
        assert_eq!(Matrix::new_diagonal(1250, 4, 0), Err(SizingError::Column(0)));
    }

    #[test]
    fn creates_matrix() {
        assert_eq!(
            Matrix::new_diagonal(12.5f32, 2, 2),
            Ok(Matrix { data: vec![12.5, 0f32, 0f32, 12.5], rows: 2, columns: 2 })
        );
        assert_eq!(
            Matrix::new_diagonal(20, 0, 0),
            Ok(Matrix { data: vec![], rows: 0, columns: 0 })
        );
        assert_eq!(
            Matrix::new_diagonal(1, 5, 3),
            Ok(Matrix {
                data: vec![1, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0],
                rows: 5,
                columns: 3,
            })
        );
        assert_eq!(
            Matrix::new_diagonal(12, 4, 5),
            Ok(Matrix {
                data: vec![12, 0, 0, 0, 0, 0, 12, 0, 0, 0, 0, 0, 12, 0, 0, 0, 0, 0, 12, 0],
                rows: 4,
                columns: 5,
            })
        );
    }
}
