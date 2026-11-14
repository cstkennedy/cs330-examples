use thiserror::Error;

#[derive(Debug, Error, PartialEq)]
pub enum PositionError {
    #[error("'{0}' is not between 0 and 10, exclusive")]
    ValueError(usize),
}

#[derive(Debug, Error, PartialEq)]
pub enum StrategyCreationError {
    // #[deprecated]
    // #[error("{0}")]
    // PositionError(#[from] PositionError),
    #[error("None of '{0:?}' are between 0 and 10, exclusive")]
    NoValidPositions(Vec<usize>),

    #[error("One of '{0:?}' is not between 0 and 10, exclusive")]
    BatchValueError(Vec<usize>),
}

#[derive(Debug, Error, PartialEq)]
pub enum StrategyError {
    #[error("{:?}", .0)]
    ParseError(#[from] std::num::ParseIntError),

    #[error("{:?}", .0)]
    PositionError(#[from] PositionError),

    #[error("Exhausted all {num_moves} prepared moves")]
    OutOfMovesError { num_moves: usize },
}

#[deprecated]
#[derive(Debug, Error, PartialEq)]
pub struct ErrorWithValue<E: std::error::Error, V> {
    #[source]
    pub the_error: E,
    pub the_value: V,
}
