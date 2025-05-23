mod accessors;
mod constructors;
mod transformers;
mod traits;
mod iter;
pub mod errors;
mod maths;
#[cfg(feature = "serde")]
mod serde;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct Matrix<T, const R: usize, const C: usize>([[T; C]; R]);