use pyo3::prelude::*;

use crate::board::Position;
use crate::error::{StrategyCreationError, StrategyError};
use crate::strategy::Strategy;


impl super::traits::DunderRepr for PyStrategy {
    type Error = std::convert::Infallible;

    type WrappedLanguage = super::traits::language::PythonForRust;

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
}

#[derive(Debug)]
pub struct PyStrategy {
    pub wrapped: Py<PyAny>, // TODO: this should not be public
}

impl Strategy for PyStrategy {
    /*
    fn next_move(&mut self) -> Result<Position, StrategyError> {
        Python::attach(|py| {
            let obj = self.wrapped.bind(py);

            // Thanks Gemini
            // println!("Type: {:?}", self.wrapped.bind(py).get_type());
            // println!("Repr: {:?}", self.wrapped.bind(py).repr().unwrap());

            // let raw_move = obj.call_method0("next_move").unwrap();
            let raw_move = obj.call_method0(pyo3::intern!(py, "next_move")).unwrap();
            // let raw_move = self.wrapped.call_method1(py, "next_move").unwrap();
            let raw_move = raw_move.extract::<usize>().unwrap();

            Ok(raw_move.try_into()?)
        })
    }
    */
    fn next_move(&mut self) -> Result<Position, StrategyError> {
        Python::attach(|py| {
            let obj = self.wrapped.bind(py);

            match obj.call_method0(pyo3::intern!(py, "next_move")) {
                Ok(raw_move) => {
                    let raw_move = raw_move.extract::<usize>()?;
                    Ok(raw_move.try_into()?)
                }
                Err(the_err) => {
                    // Method does not exist?
                    Err(StrategyError::OutOfMovesError { num_moves: 0 })
                    // TODO: This is not due to a fixed number of moves
                }
            }
        })
    }
}

impl PyStrategy {
    pub fn new(strategy: Py<PyAny>) -> Self {
        Self { wrapped: strategy }
    }
}
