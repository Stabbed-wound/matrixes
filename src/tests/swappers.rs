use super::*;

mod swap_elements {
    use super::*;

    #[test]
    fn handles_errors() {
        let mut m = Matrix::<u16>::new_identity(6);

        assert_eq!(m.swap_elements((13, 2), (5, 1)), Err(IndexError::Row(13)));
        assert_eq!(m.swap_elements((0, 8), (3, 4)), Err(IndexError::Column(8)));
        assert_eq!(m.swap_elements((18, 27), (1, 2)), Err(IndexError::Both(18, 27)));
        assert_eq!(m.swap_elements((3, 0), (6, 5)), Err(IndexError::Row(6)));
        assert_eq!(m.swap_elements((4, 3), (3, 9)), Err(IndexError::Column(9)));
        assert_eq!(m.swap_elements((0, 2), (12, 7)), Err(IndexError::Both(12, 7)));
    }

    #[test]
    fn swaps_elements() {
        let mut m = Matrix::<i8>::new_identity(3);

        assert_eq!(m.swap_elements((0, 0), (0, 2)), Ok(()));
        assert_eq!(m.data, vec![0, 0, 1, 0, 1, 0, 0, 0, 1]);
    }
}

mod swap_rows {
    use super::*;

    #[test]
    fn handles_errors() {
        let mut m = Matrix::<i64>::new_identity(7);

        assert_eq!(m.swap_rows(2, 9), Err(IndexError::Row(9)));
        assert_eq!(m.swap_rows(7, 4), Err(IndexError::Row(7)));
    }

    #[test]
    fn swaps_rows() {
        let mut m = Matrix::new_from_data(
            &vec![vec![5, 4, 3, 2, 1], vec![1, 2, 3, 4, 5], vec![5, 4, 3, 2, 1]]
        ).unwrap();

        assert_eq!(m.swap_rows(0, 1), Ok(()));
        assert_eq!(m.data, vec![1, 2, 3, 4, 5, 5, 4, 3, 2, 1, 5, 4, 3, 2, 1]);
    }
}

mod swap_columns {
    use super::*;

    #[test]
    fn handles_errors() {
        let mut m = Matrix::<u16>::new(7, 3).unwrap();

        assert_eq!(m.swap_columns(0, 4), Err(IndexError::Column(4)));
        assert_eq!(m.swap_columns(10, 2), Err(IndexError::Column(10)));
    }

    #[test]
    fn swaps_columns() {
        let mut m = Matrix::<u8>::new_identity(4);

        assert_eq!(m.swap_columns(0, 2), Ok(()));
        assert_eq!(m.data, vec![0, 0, 1, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1]);
    }
}
