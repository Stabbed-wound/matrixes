use super::*;

#[test]
fn data() {
    let m = Matrix::<u16>::new(12, 5).unwrap();

    assert_eq!(m.data(), vec![0; 60]);
}

#[test]
fn rows() {
    let m = Matrix::<u16>::new(9, 3).unwrap();

    assert_eq!(m.rows(), 9);
}

#[test]
fn columns() {
    let m = Matrix::<u16>::new(2, 4).unwrap();

    assert_eq!(m.columns(), 4);
}

#[test]
fn size() {
    let m1 = Matrix::<i32>::new(9, 16).unwrap();
    let m2 = Matrix::<u32>::new_identity(12);

    assert_eq!(m1.size(), 144);
    assert_eq!(m2.size(), 144);
}

#[test]
fn is_square() {
    let m1 = Matrix::<u16>::new(6, 2).unwrap();
    let m2 = Matrix::<i8>::new(3, 3).unwrap();
    let m3 = Matrix::<u32>::new_identity(4);

    assert!(!m1.is_square());
    assert!(m2.is_square());
    assert!(m3.is_square());
}

mod get {
    use super::*;

    #[test]
    fn handles_errors() {
        let m = Matrix::<i8>::new(5, 3).unwrap();

        assert_eq!(m.get(12, 8), Err(IndexError::Both(12, 8)));
        assert_eq!(m.get(5, 2), Err(IndexError::Row(5)));
        assert_eq!(m.get(0, 19), Err(IndexError::Column(19)));
    }

    #[test]
    fn gets_element() {
        let m = Matrix::new_from_closure(|i, j| i * j, 12, 7).unwrap();

        for r in 0..m.rows {
            for c in 0..m.columns {
                assert_eq!(m.get(r, c), Ok(&m[(r, c)]));
            }
        }
    }
}

mod get_row {
    use super::*;

    #[test]
    fn handles_errors() {
        let m = Matrix::new_from_data(&vec![vec![0]]).unwrap();

        assert_eq!(m.get_row(3), None);
    }

    #[test]
    fn gets_row() {
        let m = Matrix::<i32>::new_identity(5);

        assert_eq!(m.get_row(3).unwrap(), vec![&0i32, &0, &0, &1, &0]);
    }
}

mod get_rows {
    use super::*;

    #[test]
    fn handles_errors() {
        let m = Matrix::<u8>::new_identity(4);

        assert_eq!(m.get_rows(0..8), None);
    }

    #[test]
    fn gets_rows() {
        let m = Matrix::<u64>::new_identity(7);

        assert_eq!(
            m.get_rows(1..4).unwrap(),
            vec![
                vec![&0, &1, &0, &0, &0, &0, &0],
                vec![&0, &0, &1, &0, &0, &0, &0],
                vec![&0, &0, &0, &1, &0, &0, &0]
            ]
        )
    }
}

mod get_column {
    use super::*;

    #[test]
    fn handles_errors() {
        let m = Matrix::new_from_data(
            &vec![vec![1, 2, 3, 4, 5], vec![6, 7, 8, 7, 6], vec![5, 4, 3, 2, 1]]
        ).unwrap();

        assert_eq!(m.get_column(5), None);
    }

    #[test]
    fn gets_column() {
        let m = Matrix::new_from_data(
            &vec![vec![1, 2, 3, 4, 5], vec![6, 7, 8, 7, 6], vec![5, 4, 3, 2, 1]]
        ).unwrap();

        assert_eq!(m.get_column(3).unwrap(), vec![&4, &7, &2]);
    }
}

mod get_columns {
    use super::*;

    #[test]
    fn handles_errors() {
        let m = Matrix::new_from_data(
            &vec![vec![1, 2, 3, 4, 5], vec![6, 7, 8, 7, 6], vec![5, 4, 3, 2, 1]]
        ).unwrap();

        assert_eq!(m.get_columns(0..9), None);
    }

    #[test]
    fn gets_columns() {
        let m = Matrix::new_from_data(
            &vec![vec![1, 2, 3, 4, 5], vec![6, 7, 8, 7, 6], vec![5, 4, 3, 2, 1]]
        ).unwrap();

        assert_eq!(m.get_columns(2..4).unwrap(), vec![vec![&3, &8, &3], vec![&4, &7, &2]]);
    }
}
