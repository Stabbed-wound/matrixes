use std::ops::Range;
use matrixes::Matrix;
use divan::{ bench, black_box, Bencher };
use rand::{ distributions::{ Distribution, Standard }, random };

fn make_random_matrixes<T>(rows: Range<usize>, columns: Range<usize>) -> Vec<Matrix<T>>
    where T: Copy, Standard: Distribution<T>
{
    [(0, 0)]
        .iter()
        .chain(
            rows
                .flat_map(|rows|
                    columns
                        .to_owned()
                        .map(|columns| (rows, columns))
                        .collect::<Vec<_>>()
                )
                .collect::<Vec<_>>()
                .iter()
        )
        .map(|(rows, columns)|
            Matrix::new_from_closure(|_, _| random::<T>(), *rows, *columns).unwrap()
        )
        .collect::<Vec<_>>()
}

#[bench(types = [i32, u32, f32, f64])]
fn rows<T>(bencher: Bencher) where T: Copy, Standard: Distribution<T> {
    bencher
        .with_inputs(|| { make_random_matrixes::<T>(1..11, 1..11) })
        .bench_refs(|args| {
            for matrix in args {
                black_box(matrix.rows());
            }
        });
}

#[bench(types = [i32, u32, f32, f64])]
fn columns<T>(bencher: Bencher) where T: Copy, Standard: Distribution<T> {
    bencher
        .with_inputs(|| { make_random_matrixes::<T>(1..11, 1..11) })
        .bench_refs(|args| {
            for matrix in args {
                black_box(matrix.columns());
            }
        });
}

#[bench(types = [i32, u32, f32, f64])]
fn size<T>(bencher: Bencher) where T: Copy, Standard: Distribution<T> {
    bencher
        .with_inputs(|| { make_random_matrixes::<T>(1..11, 1..11) })
        .bench_refs(|args| {
            for matrix in args {
                black_box(matrix.size());
            }
        });
}

#[bench(types = [i32, u32, f32, f64])]
fn is_square<T>(bencher: Bencher) where T: Copy, Standard: Distribution<T> {
    bencher
        .with_inputs(|| { make_random_matrixes::<T>(1..11, 1..11) })
        .bench_refs(|args| {
            for matrix in args {
                black_box(matrix.is_square());
            }
        });
}

#[bench(types = [i32, u32, f32, f64])]
fn get<T>(bencher: Bencher) where T: Copy, Standard: Distribution<T> {
    bencher
        .with_inputs(|| { make_random_matrixes::<T>(1..11, 1..11) })
        .bench_refs(|args| {
            for matrix in args {
                for r in 0..matrix.rows() {
                    for c in 0..matrix.columns() {
                        let _ = black_box(matrix.get(black_box(r), black_box(c)));
                    }
                }
            }
        });
}

#[bench(types = [i32, u32, f32, f64])]
fn get_row<T>(bencher: Bencher) where T: Copy, Standard: Distribution<T> {
    bencher
        .with_inputs(|| { make_random_matrixes::<T>(1..11, 1..11) })
        .bench_refs(|args| {
            for matrix in args {
                for r in 0..matrix.rows() {
                    let _ = black_box(matrix.get_row(black_box(r)));
                }
            }
        });
}

#[bench(types = [i32, u32, f32, f64])]
fn get_rows<T>(bencher: Bencher) where T: Copy, Standard: Distribution<T> {
    bencher
        .with_inputs(|| { make_random_matrixes::<T>(1..11, 1..11) })
        .bench_refs(|args| {
            for matrix in args {
                for r in 0..matrix.rows() {
                    let _ = black_box(matrix.get_rows(black_box(0..r)));
                }
            }
        });
}

#[bench(types = [i32, u32, f32, f64])]
fn get_column<T>(bencher: Bencher) where T: Copy, Standard: Distribution<T> {
    bencher
        .with_inputs(|| { make_random_matrixes::<T>(1..11, 1..11) })
        .bench_refs(|args| {
            for matrix in args {
                for c in 0..matrix.columns() {
                    let _ = black_box(matrix.get_column(black_box(c)));
                }
            }
        });
}

#[bench(types = [i32, u32, f32, f64])]
fn get_columns<T>(bencher: Bencher) where T: Copy, Standard: Distribution<T> {
    bencher
        .with_inputs(|| { make_random_matrixes::<T>(1..11, 1..11) })
        .bench_refs(|args| {
            for matrix in args {
                for c in 0..matrix.columns() {
                    let _ = black_box(matrix.get_columns(black_box(0..c)));
                }
            }
        });
}

#[bench(types = [i32, u32, f32, f64])]
fn get_area<T>(bencher: Bencher) where T: Copy, Standard: Distribution<T> {
    bencher
        .with_inputs(|| {
            make_random_matrixes::<T>(1..11, 1..11)
                .iter()
                .map(|m| (
                    m.clone(),
                    (if m.rows() == 1 { 0 } else { 1 })..(if m.rows() <= 2 {
                        1
                    } else {
                        m.rows() - 2
                    }),
                    (if m.columns() == 1 { 0 } else { 1 })..(if m.columns() <= 2 {
                        1
                    } else {
                        m.columns() - 2
                    }),
                ))
                .collect::<Vec<_>>()
        })
        .bench_refs(|args| {
            for (matrix, rows, columns) in args {
                let _ = black_box(
                    matrix.get_area(black_box(rows.clone()), black_box(columns.clone()))
                );
            }
        });
}
