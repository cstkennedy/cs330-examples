use crate::board::Position;
use crate::error::StrategyError;

use crate::py_strategy::PyStrategy;
use crate::strategy::{KeyboardStrategy, PredefinedMoves, Strategy};

#[derive(Debug)]
pub enum MonoStrategy {
    Keyboard { wrapped: KeyboardStrategy },
    PredefinedMoves { wrapped: PredefinedMoves },
    PyStrategy { wrapped: PyStrategy },
}

impl Strategy for MonoStrategy {
    fn next_move(&mut self) -> Result<Position, StrategyError> {
        match self {
            MonoStrategy::Keyboard { wrapped } => wrapped.next_move(),
            MonoStrategy::PredefinedMoves { wrapped } => wrapped.next_move(),
            MonoStrategy::PyStrategy { wrapped } => wrapped.next_move(),
        }
    }
}

impl From<KeyboardStrategy> for MonoStrategy {
    fn from(wrapped: KeyboardStrategy) -> Self {
        MonoStrategy::Keyboard { wrapped }
    }
}

impl From<PredefinedMoves> for MonoStrategy {
    fn from(wrapped: PredefinedMoves) -> Self {
        MonoStrategy::PredefinedMoves { wrapped }
    }
}

impl From<PyStrategy> for MonoStrategy {
    fn from(wrapped: PyStrategy) -> Self {
        MonoStrategy::PyStrategy { wrapped }
    }
}
