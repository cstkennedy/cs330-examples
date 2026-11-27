use pyo3::prelude::*;

use crate::board::Position;
use crate::error::{StrategyCreationError, StrategyError};
use crate::strategy::Strategy;

impl super::traits::PyWrapped for Py<PyAny> {
    type Language = super::traits::language::PythonForRust;
    type WrappedBy = PyStrategy;
}

impl super::traits::DunderRepr for PyStrategy {
    /*
    type Error = std::convert::Infallible;

    fn repr_helper(&self) -> Result<String, Self::Error> {
        Python::attach(|py| {
            let obj = self.wrapped.bind(py);

            let repr_str = match obj.call_method0(pyo3::intern!(py, "__repr__")) {
                Ok(raw_value) => {
                    let repr_string = raw_value.extract::<String>().unwrap();

                    repr_string
                }
                Err(the_err) => "No __repr__".to_owned(),
            };

            Ok(repr_str)
        })
    }
    */
    type Error = PyErr;

    fn repr_helper(&self) -> Result<String, Self::Error> {
        Python::attach(|py| {
            let obj = self.wrapped.bind(py);

            let repr_str = obj
                .call_method0(pyo3::intern!(py, "__repr__"))
                .and_then(|raw_value| raw_value.extract())?;

            Ok(repr_str)
        })
    }
}

#[derive(Debug)]
pub struct PyStrategy {
    pub(crate) wrapped: Py<PyAny>, // TODO: this should not be public
}

impl Strategy for PyStrategy {
    fn next_move(&mut self) -> Result<Position, StrategyError> {
        Python::attach(|py| {
            let obj = self.wrapped.bind(py);

            /*
            match obj.call_method0(pyo3::intern!(py, "next_move")) {
                Ok(raw_move) => {
                    let the_move = raw_move.extract::<usize>()?.try_into()?;
                    Ok(the_move)
                }
                Err(the_err) => {
                    // Method does not exist?
                    Err(StrategyError::OutOfMovesError { num_moves: 0 })
                    // TODO: This is not due to a fixed number of moves
                }
            }
            */

            obj.call_method0(pyo3::intern!(py, "next_move"))
                .map_err(|_| StrategyError::OutOfMovesError { num_moves: 0 }) // TODO: Same
                .and_then(|raw_move| Ok(raw_move.extract::<usize>()?.try_into()?))
        })
    }
}

impl PyStrategy {
    pub fn new(strategy: Py<PyAny>) -> Self {
        Self { wrapped: strategy }
    }
}
