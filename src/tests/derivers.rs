use super::*;

#[test]
fn transpose() {
    let m1 = Matrix::new_from_data(&vec![vec![1, 4, 2], vec![9, 7, 1], vec![4, 6, 2]])
        .unwrap()
        .transpose();
    let m2 = Matrix::new_from_data(&vec![vec![1, 9, 4], vec![4, 7, 6], vec![2, 1, 2]]).unwrap();

    assert_eq!(m1, m2);
}

mod minor {
    use super::*;

    #[test]
    fn handles_errors() {
        let m1 = Matrix::<i32>::new(5, 6).unwrap();
        let m2 = Matrix::<i32>::new_identity(5);

        assert_eq!(m1.minor((3, 2)), Err(MinorError::NotSquare));
        assert_eq!(m1.minor((5, 7)), Err(MinorError::NotSquare));
        assert_eq!(m2.minor((7, 0)), Err(MinorError::BoundsError(IndexError::Row(7))));
        assert_eq!(m2.minor((1, 5)), Err(MinorError::BoundsError(IndexError::Column(5))));
        assert_eq!(m2.minor((6, 9)), Err(MinorError::BoundsError(IndexError::Both(6, 9))));
    }

    #[test]
    fn gets_minor() {
        let m1 = Matrix::new_from_data(&vec![vec![3, 2], vec![1, 7]]).unwrap();
        let m2 = Matrix::<u128>::new_identity(3);

        assert_eq!(m1.minor((1, 1)).unwrap(), 3);
        assert_eq!(m2.minor((0, 0)).unwrap(), 1);
    }
}

mod minor_matrix {
    use super::*;

    #[test]
    fn handles_errors() {
        let m = Matrix::new_from_data(&vec![vec![1, 2, 31]]).unwrap();

        assert_eq!(m.minor_matrix(), None);
    }

    #[test]
    fn gets_minor_matrix() {
        let m1 = Matrix::<u64>::new_from_data(&vec![vec![5, 3], vec![2, 9]]).unwrap();
        let m2 = Matrix::<i32>::new_identity(4);

        assert_eq!(m1.minor_matrix().unwrap().data, vec![9, 2, 3, 5]);
        assert_eq!(m2.minor_matrix().unwrap(), m2);
    }
}

mod cofacter {
    use super::*;

    #[test]
    fn handles_errors() {
        let m1 = Matrix::<i16>::new(5, 2).unwrap();
        let m2 = Matrix::new_from_data(&vec![vec![1, 2, 3], vec![3, 2, 1]]).unwrap();

        assert_eq!(m1.cofactor(), None);
        assert_eq!(m2.cofactor(), None);
    }

    #[test]
    fn creates_cofacter() {
        let m1 = Matrix::new_from_data(
            &vec![vec![1, 2, 3, 4], vec![5, 6, 7, 8], vec![8, 7, 6, 5], vec![4, 3, 2, 1]]
        ).unwrap();
        let m2 = Matrix::<i8>::new_identity(5);
        let m3 = Matrix::new_from_data(
            &vec![
                vec![2, 5, 0, 8, 4],
                vec![10, 2, 7, 2, 0],
                vec![8, 5, 1, 0, 6],
                vec![8, 8, 3, 3, 3],
                vec![5, 2, 0, 5, 9]
            ]
        ).unwrap();

        assert_eq!(m1.cofactor().unwrap().data, vec![0; 16]);
        assert_eq!(m2.cofactor().unwrap().data, m2.data);
        assert_eq!(
            m3.cofactor().unwrap().data,
            vec![
                -1578,
                564,
                2346,
                -885,
                1243,
                -610,
                376,
                498,
                -291,
                417,
                -2234,
                752,
                3154,
                -621,
                1419,
                2168,
                -1128,
                -3028,
                886,
                -1446,
                1468,
                -376,
                -2136,
                512,
                -1288
            ]
        );
    }
}

mod adjunct {
    use super::*;

    #[test]
    fn handles_errors() {
        let m1 = Matrix::<i16>::new(5, 2).unwrap();
        let m2 = Matrix::new_from_data(&vec![vec![1, 2, 3], vec![3, 2, 1]]).unwrap();

        assert_eq!(m1.adjunct(), None);
        assert_eq!(m2.adjunct(), None);
    }

    #[test]
    fn creates_adjunct() {
        let m1 = Matrix::new_from_data(&vec![vec![2, 7, 6], vec![3, 6, 9], vec![4, 8, 1]]).unwrap();
        let m2 = Matrix::<i32>::new_identity(4);

        assert_eq!(m1.adjunct().unwrap().data, vec![-66, 41, 27, 33, -22, 0, 0, 12, -9]);
        assert_eq!(m2.adjunct().unwrap(), m2);
    }
}

mod determinant {
    use super::*;

    #[test]
    fn handles_errors() {
        let m = Matrix::<i32>::new(7, 5).unwrap();

        assert_eq!(m.determinant(), None);
    }

    #[test]
    fn derives_determinant() {
        let m1 = Matrix::<u32>::new(4, 4).unwrap();
        let m2 = Matrix::new_from_data(&vec![vec![1, 2], vec![5, 7]]).unwrap();
        let m3 = Matrix::<i16>::new_identity(9);

        assert_eq!(m1.determinant(), Some(0));
        assert_eq!(m2.determinant(), Some(-3));
        assert_eq!(m3.determinant(), Some(1));
    }
}

mod inverse {
    use super::*;

    #[test]
    fn handles_errors() {
        let m1 = Matrix::new_from_data(&vec![vec![1, 4], vec![2, 8]]).unwrap();
        let m2 = Matrix::<i16>::new(4, 5).unwrap();

        assert_eq!(m1.inverse(), Err(InversionError::InvalidDeterminant));
        assert_eq!(m2.inverse(), Err(InversionError::NotSquare));
    }

    #[test]
    fn derives_inverse() {
        let m1 = Matrix::new_from_data(&vec![vec![1, 0, 2], vec![0, 4, 1], vec![0, 1, 0]]).unwrap();
        let m2 = Matrix::<i8>::new_identity(6);
        let m3 = Matrix::new_from_data(
            &vec![vec![1, -1, 1], vec![2, 3, 0], vec![0, -2, 1]]
        ).unwrap();

        assert_eq!(
            m1.inverse(),
            Ok(Matrix {
                data: vec![1, -2, 8, 0, 0, 1, 0, 1, -4],
                rows: 3,
                columns: 3,
            })
        );
        assert_eq!(m2.inverse(), Ok(m2));
        assert_eq!(m3.inverse().unwrap().data, vec![3, -1, -3, -2, 1, 2, -4, 2, 5]);
    }
}

mod fast_inverse {
    use super::*;

    #[test]
    fn handles_errors() {
        let m1 = Matrix::new_from_data(&vec![vec![1, 4], vec![2, 8]]).unwrap();
        let m2 = Matrix::<i16>::new(4, 5).unwrap();

        assert_eq!(m1.fast_inverse(), Err(InversionError::InvalidDeterminant));
        assert_eq!(m2.fast_inverse(), Err(InversionError::NotSquare));
    }

    #[test]
    fn derives_inverse() {
        let m1 = Matrix::new_from_data(
            &vec![vec![1f32, 0f32, 2f32], vec![0f32, 4f32, 1f32], vec![0f32, 1f32, 0f32]]
        ).unwrap();
        let m2 = Matrix::<i8>::new_identity(6);
        let m3 = Matrix::new_from_data(
            &vec![vec![1f32, -1f32, 1f32], vec![2f32, 3f32, 0f32], vec![0f32, -2f32, 1f32]]
        ).unwrap();

        assert_eq!(
            m1.fast_inverse(),
            Ok(Matrix {
                data: vec![1f32, -2f32, 8f32, 0f32, 0f32, 1f32, 0f32, 1f32, -4f32],
                rows: 3,
                columns: 3,
            })
        );
        assert_eq!(m2.fast_inverse(), Ok(m2));
        assert_eq!(
            m3
                .fast_inverse()
                .unwrap()
                .data.iter()
                .map(|f| f.round())
                .collect::<Vec<_>>(),
            vec![3f32, -1f32, -3f32, -2f32, 1f32, 2f32, -4f32, 2f32, 5f32]
        );
    }
}

mod as_resize {
    use super::*;

    #[test]
    fn handles_errors() {
        let m = Matrix::new_from_data(&vec![vec![2, 5, 3], vec![9, 1, 6]]).unwrap();

        assert_eq!(m.as_resize((0, 5)), Err(SizingError::Row(0)));
        assert_eq!(m.as_resize((2, 0)), Err(SizingError::Column(0)));
        assert_eq!(m.as_resize((3, 4)), Err(SizingError::Both(3, 4)));
    }

    #[test]
    fn resizes() {
        let m1 = Matrix::new_from_data(&vec![vec![2, 5, 3], vec![9, 1, 6]]).unwrap();
        let m2 = m1.as_resize((3, 2)).unwrap();
        let m3 = m1.as_resize((6, 1)).unwrap();
        let m4 = m1.as_resize((1, 6)).unwrap();

        assert_eq!(m2, Matrix {
            data: vec![2, 5, 3, 9, 1, 6],
            rows: 3,
            columns: 2,
        });
        assert_eq!(m3, Matrix {
            data: vec![2, 5, 3, 9, 1, 6],
            rows: 6,
            columns: 1,
        });
        assert_eq!(m4, Matrix {
            data: vec![2, 5, 3, 9, 1, 6],
            rows: 1,
            columns: 6,
        });
    }
}
