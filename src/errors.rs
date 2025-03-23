use thiserror::Error;

#[derive(Copy, Clone, Debug, Error)]
pub enum IndexError {
    #[error("Invalid row index: {0}")]
    Row(usize),
    #[error("Invalid column index: {0}")]
    Column(usize),
    #[error("Invalid row index: {0} and column index: {1}")]
    Both(usize, usize),
}

#[derive(Debug, Error)]
pub enum SquareError {
    #[error("Matrix is not square")]
    NotSquare,
}
