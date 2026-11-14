use pyo3::prelude::PyErr;
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
    PyErrCatchAll(String), // TODO: Fix with something better

    #[error("{:?}", .0)]
    ParseError(#[from] std::num::ParseIntError),

    #[error("{:?}", .0)]
    MoveError(#[from] PositionError),

    #[error("{:?}", .0)]
    OutOfMovesError(String),
}

impl From<PyErr> for StrategyError {
    fn from(py_err: PyErr) -> Self {
        StrategyError::PyErrCatchAll(py_err.to_string())
    }
}
