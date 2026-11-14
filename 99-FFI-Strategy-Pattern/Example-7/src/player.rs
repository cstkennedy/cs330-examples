use crate::board::Position;
use crate::error::StrategyError;
use crate::mono_strategy::MonoStrategy;
use crate::strategy::KeyboardStrategy;
use crate::strategy::Strategy;

#[derive(Debug)]
pub struct Player {
    name: String,
    strategy: MonoStrategy,
    humanity: bool,
}

impl Player {
    pub const DEFAULT_NAME: &'static str = "I. C. Generic";

    /// Retrieve the next move.
    pub fn next_move(&mut self) -> Result<Position, StrategyError> {
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
        &self.name
    }

    /// Checks whether a player is a placeholder or
    /// an actual player.
    ///
    /// # Args
    ///     possible_cylon (Player): player whose humanity is in question
    ///
    /// # Returns
    ///     True if the player is a Cylon
    pub fn is_generic(possible_cylon: &Player) -> bool {
        possible_cylon.name == Self::DEFAULT_NAME
    }

    pub fn builder() -> PlayerBuilder<NoName, NoStrategy, NoType> {
        PlayerBuilder::new()
    }
}

impl PartialEq for Player {
    fn eq(&self, rhs: &Self) -> bool {
        self.name == rhs.name
    }
}

#[derive(Debug, Default)]
pub struct NoStrategy;

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

    pub fn with_name(self, name: &str) -> PlayerBuilder<String, NoStrategy, NoType> {
        let name = name.to_owned();

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

impl PlayerBuilder<NoName, NoStrategy, HumanPlayer> {
    pub fn with_name(self, name: &str) -> PlayerBuilder<String, NoStrategy, HumanPlayer> {
        let name = name.to_owned();

        PlayerBuilder {
            name,
            strategy: self.strategy,
            player_type: self.player_type,
        }
    }
}

impl PlayerBuilder<String, NoStrategy, NoType> {
    pub fn with_strategy(
        self,
        strategy: impl Strategy + Into<MonoStrategy>,
    ) -> PlayerBuilder<String, MonoStrategy, NoType> {
        PlayerBuilder {
            name: self.name,
            strategy: strategy.into(),
            player_type: self.player_type,
        }
    }
}

impl PlayerBuilder<String, MonoStrategy, NoType> {
    pub fn build(self) -> Player {
        Player {
            name: self.name,
            strategy: self.strategy,
            humanity: false,
        }
    }
}

impl<'a> PlayerBuilder<String, NoStrategy, HumanPlayer> {
    pub fn build(self) -> Player {
        let strategy = KeyboardStrategy::new(&self.name).into();

        Player {
            name: self.name,
            strategy,
            humanity: true,
        }
    }
}

// TODO: implement Debug using
// <https://doc.rust-lang.org/std/fmt/struct.Formatter.html#method.debug_struct>
