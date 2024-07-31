use crate::board::{Board, VALID_SYMBOLS};
use crate::strategy::Strategy;

const DEFAULT_NAME: &str = "I. C. Generic";

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
}
/*
    pub fn __str__(self) {
        """
        Generate a player string, but only the name.
        """

        return self.name

    pub fn __deepcopy__(self, memo) -> "Player" {
        """
        Create a new duplicate Player.
        """

        cpy = Player(name=self.name, strategy=copy.deepcopy(self.strategy))

        return cpy


pub fn is_generic(possible_cylon: Player) -> bool {
    """
    Checks whether a player is a placeholder or
    an actual player.

    Args {
        possible_cylon (Player): player whose humanity is in question

    Returns {
        True if the player is a Cylon

    """

    return possible_cylon.name == DEFAULT_NAME
*/
