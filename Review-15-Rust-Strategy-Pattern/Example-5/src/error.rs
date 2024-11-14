use thiserror::Error;

#[derive(Debug, Error, PartialEq)]
pub enum BoardError {
    #[error("'new_value' is not 'X' or 'O'")]
    InvalidSymbol,
    #[error("Cell Index is not between 0 and 10, exclusive")]
    InvalidIndex,
    #[error("{0:?}")]
    Generic(&'static str),
}

#[derive(Debug, Error, PartialEq)]
#[error(transparent)]
pub struct StrategyError(BoardError);

#[derive(Debug, Error, PartialEq)]
pub struct ErrorWithValue<E: std::error::Error, V> {
    #[source]
    pub the_error: E,
    pub the_value: V,
}
