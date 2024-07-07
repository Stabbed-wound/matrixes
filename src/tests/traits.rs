use super::*;

#[test]
fn debug() {
    let m1 = Matrix::<u8>::new(1, 1).unwrap();
    let m2 = Matrix::<i16>::new(1, 7).unwrap();
    let m3 = Matrix::<u32>::new(5, 1).unwrap();
    let m4 = Matrix::<i8>::new(3, 8).unwrap();

    assert_eq!("[0]", format!("{:?}", m1));
    assert_eq!("[0, 0, 0, 0, 0, 0, 0]", format!("{:?}", m2));
    assert_eq!("[0]\n[0]\n[0]\n[0]\n[0]", format!("{:?}", m3));
    assert_eq!(
        "[0, 0, 0, 0, 0, 0, 0, 0]\n[0, 0, 0, 0, 0, 0, 0, 0]\n[0, 0, 0, 0, 0, 0, 0, 0]",
        format!("{:?}", m4)
    );
}

#[test]
fn index() {
    let m = Matrix::new_from_data(
        &vec![vec![2, 5, 7, 2, 1], vec![8, 0, 5, 3, 6], vec![6, 4, 3, 6, 7], vec![1, 7, 9, 3, 4]]
    ).unwrap();

    assert_eq!(m[(3, 4)], 4);
    assert_eq!(m[(1, 2)], 5);
    assert_eq!(m[(3, 1)], 7);
    assert_eq!(m[(0, 3)], 2);
}

#[test]
fn index_mut() {
    let mut m = Matrix::<u8>::new(3, 2).unwrap();

    m[(1, 0)] = 5;
    m[(2, 1)] = 3;
    m[(0, 0)] = 1;
    assert_eq!(m.data, vec![1, 0, 5, 0, 0, 3]);
}

#[test]
fn add_assign() {
    let mut m1 = Matrix::<i8>::new(4, 4).unwrap();
    let m2 = Matrix::<i8>::new_identity(4);
    let m3 = Matrix::new_from_data(
        &vec![vec![1, 2, 3, 4], vec![5, 6, 7, 8], vec![8, 7, 6, 5], vec![4, 3, 2, 1]]
    ).unwrap();

    m1 += m2;
    assert_eq!(m1.data, vec![1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1]);
    m1 += m3;
    assert_eq!(m1.data, vec![2, 2, 3, 4, 5, 7, 7, 8, 8, 7, 7, 5, 4, 3, 2, 2]);
}

mod add {
    use super::*;

    #[test]
    fn handles_errors() {
        let m1 = Matrix::<u8>::new(4, 4).unwrap();
        let m2 = Matrix::<u8>::new(3, 4).unwrap();
        let m3 = Matrix::<u8>::new(4, 5).unwrap();
        let m4 = Matrix::<u8>::new(3, 5).unwrap();

        assert_eq!(&m1 + &m2, Err(SizingError::Row(3)));
        assert_eq!(&m1 + &m3, Err(SizingError::Column(5)));
        assert_eq!(m1 + m4, Err(SizingError::Both(3, 5)));
    }

    #[test]
    fn adds() {
        let m = Matrix::<i8>::new_identity(4);

        assert_eq!(
            (&m + &Matrix::<i8>::new(4, 4).unwrap()).unwrap().data,
            vec![1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1]
        );
        assert_eq!(
            (
                m +
                Matrix::new_from_data(
                    &vec![vec![1, 2, 3, 4], vec![5, 6, 7, 8], vec![8, 7, 6, 5], vec![4, 3, 2, 1]]
                ).unwrap()
            ).unwrap().data,
            vec![2, 2, 3, 4, 5, 7, 7, 8, 8, 7, 7, 5, 4, 3, 2, 2]
        );
    }
}

#[test]
fn sub_assign() {
    let m1 = Matrix::<i8>::new(4, 4).unwrap();
    let mut m2 = Matrix::<i8>::new_identity(4);
    let m3 = Matrix::new_from_data(
        &vec![vec![1, 2, 3, 4], vec![5, 6, 7, 8], vec![8, 7, 6, 5], vec![4, 3, 2, 1]]
    ).unwrap();

    m2 -= m1;
    assert_eq!(m2.data, vec![1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1]);
    m2 -= m3;
    assert_eq!(m2.data, vec![0, -2, -3, -4, -5, -5, -7, -8, -8, -7, -5, -5, -4, -3, -2, 0]);
}

mod sub {
    use super::*;

    #[test]
    fn handles_errors() {
        let m1 = Matrix::<u8>::new(4, 4).unwrap();
        let m2 = Matrix::<u8>::new(3, 4).unwrap();
        let m3 = Matrix::<u8>::new(4, 5).unwrap();
        let m4 = Matrix::<u8>::new(3, 5).unwrap();

        assert_eq!(&m1 - &m2, Err(SizingError::Row(3)));
        assert_eq!(&m1 - &m3, Err(SizingError::Column(5)));
        assert_eq!(m1 - m4, Err(SizingError::Both(3, 5)));
    }

    #[test]
    fn subs() {
        let m = Matrix::<i8>::new_identity(4);

        assert_eq!(
            (&m - &Matrix::<i8>::new(4, 4).unwrap()).unwrap().data,
            vec![1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1]
        );
        assert_eq!(
            (
                m -
                Matrix::new_from_data(
                    &vec![vec![1, 2, 3, 4], vec![5, 6, 7, 8], vec![8, 7, 6, 5], vec![4, 3, 2, 1]]
                ).unwrap()
            ).unwrap().data,
            vec![0, -2, -3, -4, -5, -5, -7, -8, -8, -7, -5, -5, -4, -3, -2, 0]
        );
    }
}

mod mul {
    use super::*;

    #[test]
    fn handles_errors() {
        let m1 = Matrix::<u16>::new(2, 5).unwrap();
        let m2 = Matrix::<u16>::new(4, 2).unwrap();

        assert_eq!(m1 * m2, Err(4));
    }

    #[test]
    fn muls() {
        let m1 = Matrix::new_from_data(
            &vec![vec![1, 6, 3], vec![3, 7, 2], vec![5, 4, 8], vec![5, 6, 9]]
        ).unwrap();
        let m2 = Matrix::new_from_data(
            &vec![vec![2, 7, 5, 7], vec![9, 1, 8, 3], vec![2, 4, 6, 5]]
        ).unwrap();
        let m3 = Matrix::<i8>::new_identity(3);

        assert_eq!(
            &m1 * &m2,
            Ok(Matrix {
                data: vec![62, 25, 71, 40, 73, 36, 83, 52, 62, 71, 105, 87, 82, 77, 127, 98],
                rows: 4,
                columns: 4,
            })
        );
        assert_eq!((&m1 * &m3).unwrap(), m1);
    }
}

#[test]
fn neg() {
    let m1 = Matrix::<i32>::new_identity(3);

    assert_eq!(m1.neg().data, vec![-1, 0, 0, 0, -1, 0, 0, 0, -1]);
}

#[test]
fn default() {
    let m = Matrix::<u8>::default();

    assert_eq!(m, Matrix {
        data: vec![],
        rows: 0,
        columns: 0,
    });
}
