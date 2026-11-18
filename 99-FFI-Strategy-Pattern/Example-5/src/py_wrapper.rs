pub mod py_game;
pub mod py_player;
pub mod py_strategy;

pub mod prelude {
    pub use crate::py_wrapper::py_game::{PyCompletedGame, PyGame};
    pub use crate::py_wrapper::py_player::PyPlayer;
    pub use crate::py_wrapper::py_strategy::PyStrategy;
}

// Better approach
trait DunderRepr {
    fn repr_helper(&self) -> String;
}
