use thiserror::Error;

#[derive(Debug, Error, PartialEq)]
pub enum BoardError {
    #[error("'new_value' is not 'X' or 'O'")]
    InvalidSymbol,

    #[error("Cell Index is not between 0 and 10, exclusive")]
    InvalidIndex,
}

#[derive(Debug, Error, PartialEq)]
pub enum MoveError {
    #[error("'{0}' is not between 0 and 10, exclusive")]
    ValueError(usize),

    #[error("One of '{0:?}' is not between 0 and 10, exclusive")]
    BatchValueError(Vec<usize>),
}

#[derive(Debug, Error, PartialEq)]
pub enum StrategyCreationError {
    #[error("All Moves must be between 1 and 9, inclusive")]
    MoveError(#[from] MoveError),

    #[error("None of '{0:?}' are between 0 and 10, exclusive")]
    NoValidMoves(Vec<usize>),
}

#[derive(Debug, Error, PartialEq)]
pub enum StrategyError {
    #[error("{:?}", .0)]
    ParseError(#[from] std::num::ParseIntError),
    #[error("{:?}", .0)]
    BoardError(#[from] BoardError),
    #[error("{:?}", .0)]
    MoveError(#[from] MoveError),
    #[error("{:?}", .0)]
    OutOfMovesError(String),
}

#[derive(Debug, Error, PartialEq)]
pub struct ErrorWithValue<E: std::error::Error, V> {
    #[source]
    pub the_error: E,
    pub the_value: V,
}

#[deprecated]
type PredefinedMovesError<'a> = ErrorWithValue<StrategyError, (usize, &'a [f64])>;
