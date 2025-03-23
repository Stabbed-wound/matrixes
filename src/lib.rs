mod accessors;
mod constructors;
mod transformers;
mod traits;
mod iter;
mod errors;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Matrix<T, const R: usize, const C: usize>([[T; C]; R]);