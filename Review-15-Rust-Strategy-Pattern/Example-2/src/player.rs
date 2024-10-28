use crate::board::{Board, VALID_SYMBOLS};
use crate::strategy::Strategy;

const DEFAULT_NAME: &str = "I. C. Generic";

#[derive(Debug)]
pub struct Player<'a> {
    name: &'a str,
    strategy: Box<dyn Strategy + 'a>,
    humanity: bool,
}

impl<'a> Player<'a> {
    pub fn new(name: &'a str, strategy: impl Strategy + 'a, is_a_human: bool) -> Self {
        Player {
            name: name,
            strategy: Box::new(strategy),
            humanity: is_a_human,
        }
    }

    /// Retrieve the next move.
    pub fn next_move(&mut self) -> usize {
        self.strategy.next_move()
    }

    /// Is this a Human Player?
    pub fn is_human(&self) -> bool {
        self.humanity
    }

    /// Is this a Computer Player?
    pub fn is_computer(&self) -> bool {
        !self.is_human()
    }

    pub fn get_name(&self) -> &str {
        self.name
    }

    /// Checks whether a player is a placeholder or
    /// an actual player.
    ///
    /// # Args
    ///     possible_cylon (Player): player whose humanity is in question
    ///
    /// # Returns
    ///     True if the player is a Cylon
    pub fn is_generic(possible_cylon: &Player<'a>) -> bool {
        possible_cylon.name == DEFAULT_NAME
    }

}

// TODO: implement Debug using
// <https://doc.rust-lang.org/std/fmt/struct.Formatter.html#method.debug_struct>
