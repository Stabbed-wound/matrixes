mod accessors;
mod constructors;
mod transformers;
mod traits;
mod iter;
pub mod errors;
mod maths;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Matrix<T, const R: usize, const C: usize>([[T; C]; R]);