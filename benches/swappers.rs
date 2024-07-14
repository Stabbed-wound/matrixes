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
fn swap_elements<T>(bencher: Bencher) where T: Copy + 'static, Standard: Distribution<T> {
    bencher
        .with_inputs(|| { make_random_matrixes::<T>(2..12, 2..12) })
        .bench_refs(|args| {
            for matrix in args {
                let _ = black_box(
                    matrix.swap_elements(
                        black_box((0, 0)),
                        black_box((matrix.rows() - 1, matrix.columns() - 1))
                    )
                );
            }
        });
}

#[bench(types = [i32, u32, f32, f64])]
fn swap_rows<T>(bencher: Bencher) where T: Copy + 'static, Standard: Distribution<T> {
    bencher
        .with_inputs(|| { make_random_matrixes::<T>(3..13, 3..13) })
        .bench_refs(|args| {
            for matrix in args {
                let _ = black_box(matrix.swap_rows(0, matrix.rows() - 2));
            }
        });
}

#[bench(types = [i32, u32, f32, f64])]
fn swap_columns<T>(bencher: Bencher) where T: Copy + 'static, Standard: Distribution<T> {
    bencher
        .with_inputs(|| { make_random_matrixes::<T>(3..13, 3..13) })
        .bench_refs(|args| {
            for matrix in args {
                let _ = black_box(matrix.swap_columns(0, matrix.columns() - 2));
            }
        });
}
