mod accessors;
mod constructors;
mod swappers;
mod transformers;
mod traits;
mod iter;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Matrix<T, const R: usize, const C: usize>([[T; C]; R]);