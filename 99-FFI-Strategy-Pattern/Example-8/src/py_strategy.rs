use pyo3::prelude::*;

use crate::board::Position;
use crate::error::{StrategyCreationError, StrategyError};
use crate::strategy::Strategy;

#[derive(Debug)]
pub struct PyStrategy {
    wrapped: Py<PyAny>,
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
                    Err(StrategyError::OutOfMovesError(the_err.to_string()))
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
