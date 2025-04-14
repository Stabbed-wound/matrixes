use thiserror::Error;

#[derive(Copy, Clone, Debug, Error, Eq, PartialEq)]
pub enum IndexError {
    #[error("Invalid row index: {0}")]
    Row(usize),
    #[error("Invalid column index: {0}")]
    Column(usize),
    #[error("Invalid row index: {0} and column index: {1}")]
    Both(usize, usize),
}

#[derive(Copy, Clone, Debug, Error, Eq, PartialEq)]
pub enum TryFromSlicesError {
    #[error("Wrong number of rows: {0}")]
    Rows(usize),
    #[error("Wrong number of columns ({0}) in row: {1}")]
    Columns(usize, usize),
}
