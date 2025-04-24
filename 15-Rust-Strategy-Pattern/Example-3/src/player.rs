use crate::strategy::Strategy;
use crate::strategy::KeyboardStrategy;

pub const DEFAULT_NAME: &str = "I. C. Generic";

#[derive(Debug)]
pub struct Player<'a> {
    name: &'a str,
    strategy: Box<dyn Strategy + 'a>,
    humanity: bool,
}

impl<'a> Player<'a> {
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

    pub fn builder() -> PlayerBuilder<'a> {
        PlayerBuilder::default()
    }
}


#[derive(Debug, Default)]
pub enum PlayerType {
    Human,
    #[default] Computer
}

// TODO: Add proper error handling
#[derive(Debug, Default)]
pub struct PlayerBuilder<'a> {
    name: Option<&'a str>,
    strategy: Option<Box<dyn Strategy + 'a>>,
    player_type: Option<PlayerType>,
}

impl<'a> PlayerBuilder<'a> {
    pub fn human(mut self) -> Self {
        self.player_type = Some(PlayerType::Human);
        self
    }

    pub fn with_name(mut self, name: &'a str) -> Self {
        self.name = Some(name);

        match self.player_type {
            Some(PlayerType::Human) => {
                self.with_strategy(KeyboardStrategy::new(&name))
            },
            _ => self
        }
    }

    pub fn with_strategy(mut self, strategy: impl Strategy + 'a) -> Self {
        self.strategy = Some(Box::new(strategy));
        self
    }

    pub fn with_type(mut self, p_type: PlayerType) -> Self {
        self.player_type = Some(p_type);
        self
    }

    pub fn build(self) -> Player<'a> {
        Player {
            name: self.name.unwrap(),
            strategy: self.strategy.unwrap(),
            humanity: match self.player_type.unwrap_or(PlayerType::Computer) {
                PlayerType::Human => true,
                _ => false
            }
        }
    }

}


// TODO: implement Debug using
// <https://doc.rust-lang.org/std/fmt/struct.Formatter.html#method.debug_struct>
