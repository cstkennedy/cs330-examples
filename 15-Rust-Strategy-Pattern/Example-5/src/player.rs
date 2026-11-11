use crate::error::StrategyError;
use crate::strategy::KeyboardStrategy;
use crate::strategy::{Move, Strategy};

pub const DEFAULT_NAME: &str = "I. C. Generic";

#[derive(Debug)]
pub struct Player<'a> {
    name: &'a str,
    strategy: Box<dyn Strategy + 'a>,
    humanity: bool,
}

impl<'a> Player<'a> {
    /// Retrieve the next move.
    pub fn next_move(&mut self) -> Result<Move, StrategyError> {
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

    pub fn builder() -> PlayerBuilder<NoName, NoStrategy, NoType> {
        PlayerBuilder::new()
    }
}

impl<'a> PartialEq for Player<'a> {
    fn eq(&self, rhs: &Self) -> bool {
        self.name == rhs.name
    }
}

#[derive(Debug, Default)]
pub struct NoStrategy;

pub type BoxedStrategy<'a> = Box<dyn Strategy + 'a>;

#[derive(Debug, Default)]
pub struct HumanPlayer;

#[derive(Debug, Default)]
pub struct ComputerPlayer;

#[derive(Debug, Default)]
pub struct NoName;

#[derive(Debug, Default)]
pub struct NoType;

// TODO: Add proper error handling
#[derive(Debug)]
pub struct PlayerBuilder<N, S, T> {
    name: N,
    strategy: S,
    player_type: T,
}

impl PlayerBuilder<NoName, NoStrategy, NoType> {
    pub fn new() -> Self {
        PlayerBuilder {
            name: NoName,
            strategy: NoStrategy,
            player_type: NoType,
        }
    }

    pub fn human(self) -> PlayerBuilder<NoName, NoStrategy, HumanPlayer> {
        PlayerBuilder {
            name: NoName,
            strategy: NoStrategy,
            player_type: HumanPlayer,
        }
    }

    pub fn with_name(self, name: &str) -> PlayerBuilder<&str, NoStrategy, NoType> {
        PlayerBuilder {
            name,
            strategy: NoStrategy,
            player_type: NoType,
        }
    }
}

impl Default for PlayerBuilder<NoName, NoStrategy, NoType> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a> PlayerBuilder<NoName, NoStrategy, HumanPlayer> {
    pub fn with_name(
        self,
        name: &'a str,
    ) -> PlayerBuilder<&'a str, BoxedStrategy<'a>, HumanPlayer> {
        PlayerBuilder {
            name,
            strategy: Box::new(KeyboardStrategy::new(name)),
            player_type: self.player_type,
        }
    }
}

impl<'a> PlayerBuilder<&'a str, NoStrategy, NoType> {
    pub fn with_strategy(
        self,
        strategy: impl Strategy + 'a,
    ) -> PlayerBuilder<&'a str, BoxedStrategy<'a>, NoType> {
        PlayerBuilder {
            name: self.name,
            strategy: Box::new(strategy),
            player_type: self.player_type,
        }
    }
}

impl<'a> PlayerBuilder<&'a str, BoxedStrategy<'a>, NoType> {
    pub fn build(self) -> Player<'a> {
        Player {
            name: self.name,
            strategy: self.strategy,
            humanity: false,
        }
    }
}

impl<'a> PlayerBuilder<&'a str, BoxedStrategy<'a>, HumanPlayer> {
    pub fn build(self) -> Player<'a> {
        Player {
            name: self.name,
            strategy: self.strategy,
            humanity: true,
        }
    }
}

// TODO: implement Debug using
// <https://doc.rust-lang.org/std/fmt/struct.Formatter.html#method.debug_struct>
