use crate::board::Position;
use crate::error::StrategyError;

use crate::strategy::{KeyboardStrategy, PredefinedMoves, Strategy};

#[derive(Debug)]
pub enum MonoStrategy<'a> {
    Keyboard { wrapped: KeyboardStrategy<'a> },
    PredefinedMoves { wrapped: PredefinedMoves },
}

impl<'a> Strategy for MonoStrategy<'a> {
    fn next_move(&mut self) -> Result<Position, StrategyError> {
        match self {
            MonoStrategy::Keyboard { wrapped } => wrapped.next_move(),
            MonoStrategy::PredefinedMoves { wrapped } => wrapped.next_move(),
        }
    }
}

impl<'a> From<KeyboardStrategy<'a>> for MonoStrategy<'a> {
    fn from(wrapped: KeyboardStrategy<'a>) -> Self {
        MonoStrategy::Keyboard { wrapped }
    }
}

impl<'a> From<PredefinedMoves> for MonoStrategy<'a> {
    fn from(wrapped: PredefinedMoves) -> Self {
        MonoStrategy::PredefinedMoves { wrapped }
    }
}
