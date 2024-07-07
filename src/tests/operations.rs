use super::*;

#[test]
fn scale() {
    let mut m = Matrix::new_from_data(&vec![vec![1, 2, 3], vec![2, 3, 1], vec![3, 1, 2]]).unwrap();
    m.scale(2);

    assert_eq!(m.data, vec![2, 4, 6, 4, 6, 2, 6, 2, 4]);
}

mod scale_row {
    use super::*;

    #[test]
    fn handles_errors() {
        let mut m = Matrix::new_from_data(&vec![vec![1]]).unwrap();

        assert_eq!(m.scale_row(4, 3), Err(IndexError::Row(4)));
    }

    #[test]
    fn scales_row() {
        let mut m = Matrix::<u128>::new_identity(2);

        assert_eq!(m.scale_row(0, 3), Ok(()));
        assert_eq!(m.data, vec![3, 0, 0, 1]);
    }
}

mod add_scaled_row {
    use super::*;

    #[test]
    fn handles_errors() {
        let mut m = Matrix::new_from_data(&vec![vec![1, 2], vec![2, 1]]).unwrap();

        assert_eq!(m.add_scaled_row(2, 0, 3), Err(IndexError::Row(2)));
        assert_eq!(m.add_scaled_row(1, 5, 8), Err(IndexError::Row(5)));
    }

    #[test]
    fn adds_scaled_row() {
        let mut m = Matrix::new_from_data(
            &vec![vec![1, 0, 1], vec![2, 3, 0], vec![0, 5, 9]]
        ).unwrap();

        assert_eq!(m.add_scaled_row(0, 2, 3), Ok(()));
        assert_eq!(m.data, vec![1, 0, 1, 2, 3, 0, 3, 5, 12]);
    }
}

mod scale_column {
    use super::*;

    #[test]
    fn handles_errors() {
        let mut m = Matrix::new_from_data(&vec![vec![1]]).unwrap();

        assert_eq!(m.scale_column(4, 3), Err(IndexError::Column(4)));
    }

    #[test]
    fn scales_column() {
        let mut m = Matrix::<u128>::new_identity(2);

        assert_eq!(m.scale_column(0, 3), Ok(()));
        assert_eq!(m.data, vec![3, 0, 0, 1]);
    }
}

mod add_scaled_column {
    use super::*;

    #[test]
    fn handles_errors() {
        let mut m = Matrix::new_from_data(&vec![vec![1, 2], vec![2, 1]]).unwrap();

        assert_eq!(m.add_scaled_column(2, 0, 3), Err(IndexError::Column(2)));
        assert_eq!(m.add_scaled_column(1, 5, 8), Err(IndexError::Column(5)));
    }

    #[test]
    fn adds_scaled_column() {
        let mut m = Matrix::new_from_data(
            &vec![vec![1, 0, 1], vec![2, 3, 0], vec![0, 5, 9]]
        ).unwrap();

        assert_eq!(m.add_scaled_column(0, 2, 3), Ok(()));
        assert_eq!(m.data, vec![1, 0, 4, 2, 3, 6, 0, 5, 9]);
    }
}

mod resize {
    use super::*;

    #[test]
    fn handles_errors() {
        let mut m = Matrix::new_from_data(&vec![vec![2, 5, 3], vec![9, 1, 6]]).unwrap();

        assert_eq!(m.resize((0, 5)), Err(SizingError::Row(0)));
        assert_eq!(m.resize((2, 0)), Err(SizingError::Column(0)));
        assert_eq!(m.resize((3, 4)), Err(SizingError::Both(3, 4)));
    }

    #[test]
    fn resizes() {
        let mut m = Matrix::new_from_data(&vec![vec![2, 5, 3], vec![9, 1, 6]]).unwrap();

        assert_eq!(m.resize((3, 2)), Ok(()));
        assert_eq!(m.rows, 3);
        assert_eq!(m.columns, 2);

        assert_eq!(m.resize((6, 1)), Ok(()));
        assert_eq!(m.rows, 6);
        assert_eq!(m.columns, 1);

        assert_eq!(m.resize((1, 6)), Ok(()));
        assert_eq!(m, Matrix {
            data: vec![2, 5, 3, 9, 1, 6],
            rows: 1,
            columns: 6,
        });
    }
}

mod remove_row {
    use super::*;

    #[test]
    fn handles_errors() {
        let mut m = Matrix::<u64>::new(10, 7).unwrap();

        assert_eq!(m.remove_row(19), Err(IndexError::Row(19)));
        assert_eq!(m.remove_row(10), Err(IndexError::Row(10)));
    }

    #[test]
    fn removes_row() {
        let mut m = Matrix::new_from_data(
            &vec![
                vec![1, 2, 3],
                vec![4, 5, 6],
                vec![7, 8, 9],
                vec![10, 11, 10],
                vec![9, 8, 7],
                vec![6, 5, 4],
                vec![3, 2, 1]
            ]
        ).unwrap();

        assert_eq!(m.remove_row(2), Ok(()));
        assert_eq!(m, Matrix {
            data: vec![1, 2, 3, 4, 5, 6, 10, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1],
            rows: 6,
            columns: 3,
        });
        assert_eq!(m.remove_row(5), Ok(()));
        assert_eq!(m, Matrix {
            data: vec![1, 2, 3, 4, 5, 6, 10, 11, 10, 9, 8, 7, 6, 5, 4],
            rows: 5,
            columns: 3,
        });
    }
}

mod remove_column {
    use super::*;

    #[test]
    fn handles_errors() {
        let mut m = Matrix::new_with_data(
            5,
            vec![3, 5, 4, 6, 8, 6, 3, 2, 1, 6, 8, 5, 8, 4, 5, 6, 7, 3, 4, 0]
        ).unwrap();

        assert_eq!(m.remove_column(0), Ok(()));
        assert_eq!(m, Matrix {
            data: vec![5, 4, 6, 8, 3, 2, 1, 6, 5, 8, 4, 5, 7, 3, 4, 0],
            rows: 4,
            columns: 4,
        });
        assert_eq!(m.remove_column(2), Ok(()));
        assert_eq!(m, Matrix {
            data: vec![5, 4, 8, 3, 2, 6, 5, 8, 5, 7, 3, 0],
            rows: 4,
            columns: 3,
        });
    }
}

mod insert_row {
    use super::*;

    #[test]
    fn handles_errors() {
        let mut m = Matrix::<u128>::new(4, 7).unwrap();

        assert_eq!(m.insert_row(9, &vec![1, 2, 3, 4, 5, 6, 7]), Err(SizingError::Row(9)));
        assert_eq!(
            m.insert_row(3, &vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]),
            Err(SizingError::Column(10))
        );
        assert_eq!(
            m.insert_row(20, &vec![3, 5, 12, 3, 56, 7, 8, 4, 2, 1, 6, 23, 1, 7]),
            Err(SizingError::Both(20, 14))
        );
    }

    #[test]
    fn inserts_row() {
        let mut m = Matrix::<i8>
            ::new_from_data(&vec![vec![1, 2, 3, 4, 5], vec![6, 7, 8, 7, 6], vec![5, 4, 3, 2, 1]])
            .unwrap();

        assert_eq!(m.insert_row(0, &vec![10, 9, 8, 7, 6]), Ok(()));
        assert_eq!(m, Matrix {
            data: vec![10, 9, 8, 7, 6, 1, 2, 3, 4, 5, 6, 7, 8, 7, 6, 5, 4, 3, 2, 1],
            rows: 4,
            columns: 5,
        });
        assert_eq!(m.insert_row(4, &vec![2, 4, 6, 8, 10]), Ok(()));
        assert_eq!(m, Matrix {
            data: vec![10, 9, 8, 7, 6, 1, 2, 3, 4, 5, 6, 7, 8, 7, 6, 5, 4, 3, 2, 1, 2, 4, 6, 8, 10],
            rows: 5,
            columns: 5,
        });
    }
}

mod insert_column {
    use super::*;

    #[test]
    fn handles_errors() {
        let mut m = Matrix::<u16>::new(19, 7).unwrap();

        assert_eq!(m.insert_column(2, &vec![0, 3, 1]), Err(SizingError::Row(3)));
        assert_eq!(m.insert_column(32, &(0..19).collect::<Vec<_>>()), Err(SizingError::Column(32)));
        assert_eq!(
            m.insert_column(22, &vec![2, 4, 6, 8, 1, 3, 5, 7, 9]),
            Err(SizingError::Both(9, 22))
        );
    }

    #[test]
    fn inserts_column() {
        let mut m = Matrix::new_from_data(
            &vec![vec![1, 4, 7, 10], vec![2, 5, 8, 11], vec![3, 6, 9, 12]]
        ).unwrap();

        assert_eq!(m.insert_column(2, &vec![0, 0, 0]), Ok(()));
        assert_eq!(m, Matrix {
            data: vec![1, 4, 0, 7, 10, 2, 5, 0, 8, 11, 3, 6, 0, 9, 12],
            rows: 3,
            columns: 5,
        });
        assert_eq!(m.insert_column(2, &vec![0, 0, 0]), Ok(()));
        assert_eq!(m, Matrix {
            data: vec![1, 4, 0, 0, 7, 10, 2, 5, 0, 0, 8, 11, 3, 6, 0, 0, 9, 12],
            rows: 3,
            columns: 6,
        });
    }
}

mod join_matrix_above {
    use super::*;

    #[test]
    fn handles_errors() {
        let mut m1 = Matrix::<i8>::new(2, 9).unwrap();
        let m2 = Matrix::new(3, 5).unwrap();

        assert_eq!(m1.join_matrix_above(&m2), Err(SizingError::Column(5)));
    }

    #[test]
    fn joins_above() {
        let mut m1 = Matrix::new_identity(2);
        let m2 = Matrix::new_with_data(2, vec![2, 5]).unwrap();

        assert_eq!(m1.join_matrix_above(&m2), Ok(()));
        assert_eq!(m1, Matrix {
            data: vec![2, 5, 1, 0, 0, 1],
            columns: 2,
            rows: 3,
        });
    }
}

mod join_matrix_below {
    use super::*;

    #[test]
    fn handles_errors() {
        let mut m1 = Matrix::<i8>::new(2, 2).unwrap();
        let m2 = Matrix::new(3, 7).unwrap();

        assert_eq!(m1.join_matrix_below(&m2), Err(SizingError::Column(7)));
    }

    #[test]
    fn joins_below() {
        let mut m1 = Matrix::new_identity(2);
        let m2 = Matrix::new_with_data(2, vec![2, 5]).unwrap();

        assert_eq!(m1.join_matrix_below(&m2), Ok(()));
        assert_eq!(m1, Matrix {
            data: vec![1, 0, 0, 1, 2, 5],
            columns: 2,
            rows: 3,
        });
    }
}

mod join_matrix_left {
    use super::*;

    #[test]
    fn handles_errors() {
        let mut m1 = Matrix::<i8>::new(5, 1).unwrap();
        let m2 = Matrix::new(3, 2).unwrap();

        assert_eq!(m1.join_matrix_left(&m2), Err(SizingError::Row(3)));
    }

    #[test]
    fn joins_left() {
        let mut m1 = Matrix::new_identity(2);
        let m2 = Matrix::new_with_data(1, vec![2, 5]).unwrap();

        assert_eq!(m1.join_matrix_left(&m2), Ok(()));
        assert_eq!(m1, Matrix {
            data: vec![2, 1, 0, 5, 0, 1],
            columns: 3,
            rows: 2,
        });
    }
}

mod join_matrix_right {
    use super::*;

    #[test]
    fn handles_errors() {
        let mut m1 = Matrix::<i8>::new(8, 5).unwrap();
        let m2 = Matrix::new(11, 11).unwrap();

        assert_eq!(m1.join_matrix_right(&m2), Err(SizingError::Row(11)));
    }

    #[test]
    fn joins_left() {
        let mut m1 = Matrix::new_identity(2);
        let m2 = Matrix::new_with_data(1, vec![2, 5]).unwrap();

        assert_eq!(m1.join_matrix_right(&m2), Ok(()));
        assert_eq!(m1, Matrix {
            data: vec![1, 0, 2, 0, 1, 5],
            columns: 3,
            rows: 2,
        });
    }
}
