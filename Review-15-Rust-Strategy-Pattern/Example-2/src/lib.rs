pub mod board;
pub mod game;
pub mod player;
pub mod referee;
pub mod strategy;

pub mod prelude {
    pub use crate::game::Game;
    pub use crate::player::Player;
    pub use crate::strategy::*;
}
