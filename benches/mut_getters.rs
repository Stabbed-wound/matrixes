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
fn get_mut<T>(bencher: Bencher) where T: Copy, Standard: Distribution<T> {
    bencher
        .with_inputs(|| { make_random_matrixes::<T>(1..11, 1..11) })
        .bench_refs(|args| {
            for matrix in args {
                for r in 0..matrix.rows() {
                    for c in 0..matrix.columns() {
                        let _ = black_box(matrix.get_mut(black_box(r), black_box(c)));
                    }
                }
            }
        });
}

#[bench(types = [i32, u32, f32, f64])]
fn get_mut_row<T>(bencher: Bencher) where T: Copy, Standard: Distribution<T> {
    bencher
        .with_inputs(|| { make_random_matrixes::<T>(1..11, 1..11) })
        .bench_refs(|args| {
            for matrix in args {
                for r in 0..matrix.rows() {
                    let _ = black_box(matrix.get_mut_row(black_box(r)));
                }
            }
        });
}

#[bench(types = [i32, u32, f32, f64])]
fn get_mut_column<T>(bencher: Bencher) where T: Copy, Standard: Distribution<T> {
    bencher
        .with_inputs(|| { make_random_matrixes::<T>(1..11, 1..11) })
        .bench_refs(|args| {
            for matrix in args {
                for c in 0..matrix.columns() {
                    let _ = black_box(matrix.get_mut_column(black_box(c)));
                }
            }
        });
}
