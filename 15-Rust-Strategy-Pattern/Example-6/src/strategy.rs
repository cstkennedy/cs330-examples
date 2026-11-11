use std::fmt::Debug;
use std::io::Write;
use std::io::{stdin, stdout};

use crate::error::{MoveError, StrategyCreationError, StrategyError};
use crate::r#move::Move;


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
    fn read_line_as_string() -> Result<String, std::io::Error> {
        let mut str_buffer = String::new();
        let _ = stdin().read_line(&mut str_buffer)?;
        let str_buffer = str_buffer.trim().to_string();

        Ok(str_buffer)
    }
}

impl<'a> Strategy for KeyboardStrategy<'a> {
    fn next_move(&mut self) -> Result<Move, StrategyError> {
        print!("{}, {}", &self.name, Self::PROMPT_MSG);
        let _ = stdout().flush();

        let choice: usize = Self::read_line_as_string().unwrap().parse()?;

        Ok(choice.try_into()?)
    }
}

#[derive(Debug)]
pub struct PredefinedMoves {
    my_moves: Vec<Move>,
    move_idx: usize,
}

impl PredefinedMoves {
    pub fn from_iterable_discarding<I>(raw_moves: I) -> Result<PredefinedMoves, StrategyCreationError>
        where I: IntoIterator<Item=usize>
    {
        let some_moves: Vec<usize> = raw_moves.into_iter().collect();

        let validated_moves: Vec<Move> = some_moves.iter()
            .copied()
            .flat_map(Move::try_from)
            .collect();

        if validated_moves.is_empty() {
            return Err(StrategyCreationError::NoValidMoves(some_moves.into()));
        }

        Ok(PredefinedMoves {
            my_moves: validated_moves,
            move_idx: 0,
        })
    }

    pub fn from_iterable_nondiscarding<I>(raw_moves: I) -> Result<PredefinedMoves, StrategyCreationError>
        where I: IntoIterator<Item=usize>
    {
        let some_moves: Vec<usize> = raw_moves.into_iter().collect();

        let validated_moves: Vec<Move> = some_moves.iter()
            .copied()
            .flat_map(Move::try_from)
            .collect();

        if validated_moves.is_empty() {
            return Err(StrategyCreationError::NoValidMoves(some_moves.into()));
        }

        if validated_moves.len() != some_moves.len() {
            return Err(MoveError::BatchValueError(some_moves.into()).into());
        }

        Ok(PredefinedMoves {
            my_moves: validated_moves,
            move_idx: 0,
        })
    }
}

impl Strategy for PredefinedMoves {
    fn next_move(&mut self) -> Result<Move, StrategyError> {
        if self.move_idx == self.my_moves.len() {
            return Err(StrategyError::OutOfMovesError(format!(
                "Exhausted all {} prepared moves",
                self.my_moves.len()
            )));
        }

        let my_next_move = self.my_moves[self.move_idx];
        self.move_idx += 1;

        Ok(my_next_move)
    }
}
