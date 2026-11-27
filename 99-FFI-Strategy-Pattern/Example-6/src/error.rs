use pyo3::prelude::PyErr;
use thiserror::Error;

#[derive(Debug, Error, PartialEq)]
pub enum PositionError {
    #[error("'{0}' is not between 0 and 10, exclusive")]
    ValueError(usize),
}

#[derive(Debug, Error, PartialEq)]
pub enum StrategyCreationError {
    #[error("None of '{0:?}' are between 0 and 10, exclusive")]
    NoValidPositions(Vec<usize>),

    #[error("One of '{0:?}' is not between 0 and 10, exclusive")]
    BatchValueError(Vec<usize>),
}

// #[derive(Debug, Error, PartialEq)]
// #[derive(Debug, Error)]
#[derive(Debug, Error, PartialEq)]
pub enum StrategyError {
    #[error("{:?}", .0)]
    // PyCatchAllError(PyErr), // TODO: Fix with something (that is actually) better
    PyCatchAllError(#[from] PyCatchAllError),

    #[error("{:?}", .0)]
    ParseError(#[from] std::num::ParseIntError),

    #[error("{:?}", .0)]
    PositionError(#[from] PositionError),

    #[error("Exhausted all {num_moves} prepared moves")]
    OutOfMovesError { num_moves: usize },
}

/*
impl From<PyErr> for StrategyError {
    fn from(py_err: PyErr) -> Self {
        StrategyError::PyCatchAllError(py_err)
    }
}
*/

impl From<PyErr> for PyCatchAllError {
    fn from(py_err: PyErr) -> Self {
        PyCatchAllError { wrapped: py_err }
    }
}

impl From<PyErr> for StrategyError {
    fn from(py_err: PyErr) -> Self {
        py_err.into()
    }
}

/*
impl PartialEq for StrategyError {
    fn eq(&self, rhs: &Self) -> bool {
        match (self, rhs) {
            (Self::PyCatchAllError(_lhs_err), Self::PyCatchAllError(_rhs_err)) => todo!(),
            (Self::ParseError(lhs_err), Self::ParseError(rhs_err)) => lhs_err == rhs_err,
            (Self::PositionError(lhs_err), Self::PositionError(rhs_err)) => lhs_err == rhs_err,
            (
                Self::OutOfMovesError { num_moves: lhs },
                Self::OutOfMovesError { num_moves: rhs },
            ) => lhs == rhs,
            _ => false,
        }
    }
}
*/

#[derive(Debug, Error)]
#[error(transparent)]
pub struct PyCatchAllError {
    wrapped: PyErr,
}

impl PartialEq for PyCatchAllError {
    fn eq(&self, rhs: &Self) -> bool {
        // todo!();
        false
    }
}
