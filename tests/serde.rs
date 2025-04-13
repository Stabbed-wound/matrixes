#![cfg(feature = "serde")]

use matrixes::Matrix;
use num_traits::real::Real;
use rstest::*;
use rstest_reuse::{apply, template};
use serde::Deserialize;
use ::serde::Serialize;
use std::fmt::Debug;

#[fixture]
fn one_by_one() -> Matrix<i32, 1, 1> {
    Matrix::identity()
}

#[fixture]
fn quite_large() -> Matrix<f32, 20, 18> {
    Matrix::from_fn(|i, j| 1.25.powi(i.try_into().expect("")) * 0.8.powi(j.try_into().expect("")))
}

#[fixture]
fn empty() -> Matrix<u128, 0, 0> {
    Matrix::new()
}

#[fixture]
fn no_rows() -> Matrix<i32, 0, 2000> {
    Matrix::new()
}

fn no_cols() -> Matrix<u16, 23, 0> {
    Matrix::new()
}

#[template]
#[rstest]
#[case(one_by_one())]
#[case(quite_large())]
#[case(empty())]
#[case(no_rows())]
#[case(no_cols())]
fn serde<T, const R: usize, const C: usize>(#[case] matrix: Matrix<T, R, C>) {}

#[apply(serde)]
fn serialize<T, const R: usize, const C: usize>(#[case] matrix: Matrix<T, R, C>)
where
    T: Serialize + Copy + Debug,
{
    let ser = serde_json::to_string(&matrix).expect("");
    dbg!(ser);
}

#[apply(serde)]
fn deserialize<T, const R: usize, const C: usize>(#[case] matrix: Matrix<T, R, C>)
where
    T: Serialize + for<'a> Deserialize<'a> + Copy + Debug + PartialEq,
{
    let ser = serde_json::to_string(&matrix).expect("");
    let de: Matrix<T, R, C> = serde_json::from_str(&ser).expect("");
    assert_eq!(matrix, de);
}
