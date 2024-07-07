use super::*;

mod constructors {
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
                    data: vec![
                        0,
                        1,
                        2,
                        3,
                        4,
                        5,
                        1,
                        2,
                        3,
                        4,
                        5,
                        6,
                        2,
                        3,
                        4,
                        5,
                        6,
                        7,
                        3,
                        4,
                        5,
                        6,
                        7,
                        8
                    ],
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

    mod new_diagonal_matrix {
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
}

mod getters {
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
}

mod mut_getters {
    use super::*;

    #[test]
    fn data_mut() {
        let mut m = Matrix::new_identity(2);
        let data = m.data_mut();

        assert_eq!(data, &mut vec![1, 0, 0, 1]);

        data[1] = 5;

        assert_eq!(m.data, vec![1, 5, 0, 1]);
    }

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
}

mod swappers {
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
}

mod operations {
    use super::*;

    #[test]
    fn scale() {
        let mut m = Matrix::new_from_data(
            &vec![vec![1, 2, 3], vec![2, 3, 1], vec![3, 1, 2]]
        ).unwrap();
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
                ::new_from_data(
                    &vec![vec![1, 2, 3, 4, 5], vec![6, 7, 8, 7, 6], vec![5, 4, 3, 2, 1]]
                )
                .unwrap();

            assert_eq!(m.insert_row(0, &vec![10, 9, 8, 7, 6]), Ok(()));
            assert_eq!(m, Matrix {
                data: vec![10, 9, 8, 7, 6, 1, 2, 3, 4, 5, 6, 7, 8, 7, 6, 5, 4, 3, 2, 1],
                rows: 4,
                columns: 5,
            });
            assert_eq!(m.insert_row(4, &vec![2, 4, 6, 8, 10]), Ok(()));
            assert_eq!(m, Matrix {
                data: vec![
                    10,
                    9,
                    8,
                    7,
                    6,
                    1,
                    2,
                    3,
                    4,
                    5,
                    6,
                    7,
                    8,
                    7,
                    6,
                    5,
                    4,
                    3,
                    2,
                    1,
                    2,
                    4,
                    6,
                    8,
                    10
                ],
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
            assert_eq!(
                m.insert_column(32, &(0..19).collect::<Vec<_>>()),
                Err(SizingError::Column(32))
            );
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
}

mod derivers {
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
            let m1 = Matrix::new_from_data(
                &vec![vec![2, 7, 6], vec![3, 6, 9], vec![4, 8, 1]]
            ).unwrap();
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
            let m1 = Matrix::new_from_data(
                &vec![vec![1, 0, 2], vec![0, 4, 1], vec![0, 1, 0]]
            ).unwrap();
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
}

mod traits {
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
            &vec![
                vec![2, 5, 7, 2, 1],
                vec![8, 0, 5, 3, 6],
                vec![6, 4, 3, 6, 7],
                vec![1, 7, 9, 3, 4]
            ]
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
                        &vec![
                            vec![1, 2, 3, 4],
                            vec![5, 6, 7, 8],
                            vec![8, 7, 6, 5],
                            vec![4, 3, 2, 1]
                        ]
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
                        &vec![
                            vec![1, 2, 3, 4],
                            vec![5, 6, 7, 8],
                            vec![8, 7, 6, 5],
                            vec![4, 3, 2, 1]
                        ]
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
}
