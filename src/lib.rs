mod constructors;
mod getters;
mod mut_getters;
mod swappers;
mod transformers;
mod traits;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Matrix<T, const R: usize, const C: usize>([[T; C]; R]);