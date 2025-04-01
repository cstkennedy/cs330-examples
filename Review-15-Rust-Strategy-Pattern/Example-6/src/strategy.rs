use std::fmt::Debug;
use std::io::Write;
use std::io::{stdin, stdout};

use crate::error::{MoveError, StrategyCreationError, StrategyError};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Move(usize);

impl From<Move> for usize {
    fn from(val: Move) -> usize {
        val.0
    }
}

impl TryFrom<usize> for Move {
    type Error = MoveError;

    fn try_from(val: usize) -> Result<Move, Self::Error> {
        if val < 1 || val > 9 {
            return Err(MoveError::ValueError(val));
        }

        Ok(Move(val))
    }
}

pub trait Strategy: Debug {
    fn next_move(&mut self) -> Result<Move, StrategyError>;
}

#[derive(Debug)]
pub struct KeyboardStrategy<'a> {
    name: &'a str,
}

impl<'a> KeyboardStrategy<'a> {
    const PROMPT_MSG: &'static str = "Enter your desired move (1-9): ";

    pub fn new(player_name: &'a str) -> Self {
        KeyboardStrategy { name: player_name }
    }

    /// Read a line from std in and trim the trailing newline.
    fn read_line_as_string() -> String {
        let mut str_buffer = String::new();
        let _ = stdin().read_line(&mut str_buffer).unwrap();
        let str_buffer = str_buffer.trim().to_string();

        str_buffer
    }
}

impl<'a> Strategy for KeyboardStrategy<'a> {
    fn next_move(&mut self) -> Result<Move, StrategyError> {
        print!("{}, {}", &self.name, Self::PROMPT_MSG);
        let _ = stdout().flush();

        let choice: usize = Self::read_line_as_string().parse()?;

        Ok(choice.try_into()?)
    }
}

#[derive(Debug)]
pub struct PredefinedMoves<'a> {
    my_moves: &'a [usize],
    move_idx: usize,
}

/*
impl<'a> PredefinedMoves<'a> {
    pub fn new(some_moves: &'a [usize]) -> Self {
        PredefinedMoves {
            my_moves: some_moves,
            move_idx: 0,
        }
    }
}
*/

impl<'a> TryFrom<&'a [usize]> for PredefinedMoves<'a> {
    type Error = StrategyCreationError;

    fn try_from(some_moves: &'a [usize]) -> Result<PredefinedMoves<'a>, Self::Error> {
        for entry in some_moves.iter() {
            let _ = Move::try_from(*entry)?;
        }

        Ok(PredefinedMoves {
            my_moves: some_moves,
            move_idx: 0,
        })
    }
}

impl<'a, const N: usize> TryFrom<&'a [usize; N]> for PredefinedMoves<'a> {
    type Error = StrategyCreationError;

    fn try_from(some_moves: &'a [usize; N]) -> Result<PredefinedMoves<'a>, Self::Error> {
        for entry in some_moves.iter() {
            let _ = Move::try_from(*entry)?;
        }

        Ok(PredefinedMoves {
            my_moves: some_moves,
            move_idx: 0,
        })
    }
}

impl<'a> Strategy for PredefinedMoves<'a> {
    fn next_move(&mut self) -> Result<Move, StrategyError> {
        if self.move_idx == self.my_moves.len() {
            return Err(StrategyError::OutOfMovesError(format!(
                "Exhausted all {} prepared moves",
                self.my_moves.len()
            )));
        }

        let my_next_move = self.my_moves[self.move_idx];
        self.move_idx += 1;

        Ok(my_next_move.try_into()?)
    }
}
