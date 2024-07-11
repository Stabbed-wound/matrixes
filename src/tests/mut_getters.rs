use super::*;

mod get_mut {
    use super::*;

    #[test]
    fn handles_errors() {
        let mut m = Matrix::<i8>::new(5, 3).unwrap();

        assert_eq!(m.get_mut(12, 8), Err(IndexError::Both(12, 8)));
        assert_eq!(m.get_mut(5, 2), Err(IndexError::Row(5)));
        assert_eq!(m.get_mut(0, 19), Err(IndexError::Column(19)));
    }

    #[test]
    fn gets_element() {
        let mut m = Matrix::new_from_closure(|i, j| i * j, 12, 7).unwrap();

        for r in 0..m.rows {
            for c in 0..m.columns {
                *m.get_mut(r, c).unwrap() = 13;
            }
        }

        for r in 0..m.rows {
            for c in 0..m.columns {
                assert_eq!(m[(r, c)], 13);
            }
        }
    }
}

mod get_mut_row {
    use super::*;

    #[test]
    fn handles_errors() {
        let mut m = Matrix::<i128>::new(7, 5).unwrap();

        assert_eq!(m.get_mut_row(23), None);
    }

    #[test]
    fn gets_mut_row() {
        let mut m = Matrix::<i8>::new_identity(3);
        let mut row = m.get_mut_row(2).unwrap();

        assert_eq!(row, vec![&mut 0, &mut 0, &mut 1]);

        *row[0] = 3;

        assert_eq!(m.get_row(2).unwrap(), vec![&3, &0, &1]);
    }
}

mod get_mut_column {
    use super::*;

    #[test]
    fn handles_errors() {
        let mut m = Matrix::<u8>::new(5, 8).unwrap();

        assert_eq!(m.get_mut_column(19), None);
    }

    #[test]
    fn gets_column() {
        let mut m = Matrix::new_from_data(
            &vec![
                vec![1, 2, 3, 4, 5, 6, 7],
                vec![8, 9, 10, 11, 12, 13, 14],
                vec![14, 13, 12, 11, 10, 9, 8],
                vec![7, 6, 5, 4, 3, 2, 1]
            ]
        ).unwrap();
        let mut col = m.get_mut_column(5).unwrap();

        assert_eq!(col, vec![&mut 6, &mut 13, &mut 9, &mut 2]);

        *col[3] = 17;

        assert_eq!(m.get_column(5).unwrap(), vec![&6, &13, &9, &17]);
    }
}
