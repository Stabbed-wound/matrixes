use matrixes::Matrix;
use divan::{ bench, black_box, Bencher };
use rand::{ distributions::{ Distribution, Standard }, random };
use num::{ One, Zero };

#[bench(types = [i32, u32, f32, f64])]
fn empty<T>() where T: Copy {
    black_box(Matrix::<T>::empty());
}

#[bench(types = [i32, u32, f32, f64])]
fn new<T>(bencher: Bencher) where T: Copy + Zero {
    bencher
        .with_inputs(|| {
            [(0, 0)]
                .iter()
                .chain(
                    (1..11 as usize)
                        .flat_map(|rows|
                            (1..11 as usize).map(|cols| (rows, cols)).collect::<Vec<_>>()
                        )
                        .collect::<Vec<_>>()
                        .iter()
                )
                .cloned()
                .collect::<Vec<_>>()
        })
        .bench_refs(|iterator| {
            for (r, c) in iterator {
                let _ = black_box(Matrix::<T>::new(black_box(*r), black_box(*c)));
            }
        });
}

#[bench(types = [i32, u32, f32, f64])]
fn new_identity<T>(bencher: Bencher) where T: Copy + Zero + One {
    bencher
        .with_inputs(|| { 0..100 as usize })
        .bench_refs(|range| {
            for n in range {
                let _ = black_box(Matrix::<T>::new_identity(black_box(n)));
            }
        });
}

#[bench(types = [i32, u32, f32, f64])]
fn new_with_data<T>(bencher: Bencher) where T: Copy, Standard: Distribution<T> {
    bencher
        .with_inputs(|| {
            [(0 as usize, vec![])]
                .iter()
                .chain(
                    (1..11 as usize)
                        .flat_map(|cols| {
                            (1..11)
                                .map(|rows| (cols, (0..rows * cols).map(|_| random()).collect()))
                                .collect::<Vec<_>>()
                        })
                        .collect::<Vec<_>>()
                        .iter()
                )
                .cloned()
                .collect::<Vec<_>>()
        })
        .bench_refs(|args: &mut Vec<(usize, Vec<T>)>| {
            for (columns, data) in args.iter() {
                let _ = black_box(
                    Matrix::new_with_data(black_box(*columns), black_box(data.to_owned()))
                );
            }
        });
}

#[bench(types = [i32, u32, f32, f64])]
fn new_from_data<T>(bencher: Bencher) where T: Copy, Standard: Distribution<T> {
    bencher
        .with_inputs(|| {
            [vec![]]
                .iter()
                .chain(
                    (1..11)
                        .flat_map(|cols| {
                            (1..11)
                                .map(|rows| {
                                    (0..rows)
                                        .map(|_|
                                            (0..cols).map(|_| random::<T>()).collect::<Vec<_>>()
                                        )
                                        .collect::<Vec<_>>()
                                })
                                .collect::<Vec<_>>()
                        })
                        .collect::<Vec<_>>()
                        .iter()
                )
                .cloned()
                .collect::<Vec<_>>()
        })
        .bench_refs(|args: &mut Vec<Vec<Vec<T>>>| {
            for elements in args {
                let _ = black_box(Matrix::new_from_data(black_box(elements)));
            }
        })
}

#[bench(types = [i32, u32, f32, f64])]
fn new_from_closure<T>(bencher: Bencher) where T: Copy + One + Zero, Standard: Distribution<T> {
    bencher
        .with_inputs(|| {
            [
                |i: usize, j: usize| { (0..i + j).fold(T::zero(), |prev, _| prev + T::one()) },
                |i: usize, j: usize| if (i + j) % 2 == 0 { T::one() } else { T::zero() },
                |i: usize, j: usize| (
                    if i >= j {
                        T::zero()
                    } else {
                        (0..j.checked_sub(i).unwrap_or(0)).fold(
                            T::zero(),
                            |prev, _| prev + T::one()
                        )
                    }
                ),
                |_, _| random(),
            ]
                .iter()
                .flat_map(|f|
                    [(f, 0, 0)]
                        .iter()
                        .chain(
                            (1..6 as usize)
                                .flat_map(|rows|
                                    (1..6 as usize)
                                        .map(|columns| (f, rows, columns))
                                        .collect::<Vec<_>>()
                                )
                                .collect::<Vec<_>>()
                                .iter()
                        )
                        .cloned()
                        .collect::<Vec<_>>()
                )
                .collect::<Vec<_>>()
        })
        .bench_refs(|args| {
            for (f, rows, columns) in args {
                let _ = black_box(
                    Matrix::new_from_closure(black_box(*f), black_box(*rows), black_box(*columns))
                );
            }
        });
}

#[bench(types = [i32, u32, f32, f64])]
fn new_diagonal<T>(bencher: Bencher) where T: Copy + Zero, Standard: Distribution<T> {
    bencher
        .with_inputs(|| {
            [(random(), 0, 0)]
                .iter()
                .chain(
                    (1..11)
                        .flat_map(|rows| {
                            (1..11).map(|columns| (random(), rows, columns)).collect::<Vec<_>>()
                        })
                        .collect::<Vec<_>>()
                        .iter()
                )
                .cloned()
                .collect::<Vec<_>>()
        })
        .bench_refs(|args| {
            for (value, rows, columns) in args {
                let _ = black_box(
                    Matrix::new_diagonal(black_box(*value), black_box(*rows), black_box(*columns))
                );
            }
        });
}
