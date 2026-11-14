use pyo3::prelude::*;

pub mod board;
pub mod error;
pub mod game;
pub mod mono_strategy;
pub mod player;
pub mod referee;
pub mod strategy;

pub mod prelude {
    pub use crate::game::Game;
    pub use crate::player::Player;
    pub use crate::strategy::*;
}

use crate::player::Player;
use crate::game::{Game, CompletedGame};
use crate::strategy::*;


#[pyclass(name="Player")]
pub struct PyPlayer {
    player: Option<Player>
}

impl From<Player> for PyPlayer {
    fn from(player: Player) -> PyPlayer {
        PyPlayer{player: Some(player)}
    }
}

#[pymethods]
impl PyPlayer {
    #[rustfmt::skip]
    #[staticmethod]
    pub fn create_human(name: &str) -> Self {
        Player::builder()
            .human()
            .with_name(name)
            .build()
            .into()
    }

    /*
    pub fn create_computer<S>(name: &str, strategy: S) -> Player
    where
        S: Strategy + Into<MonoStrategy>,
    */
    #[rustfmt::skip]
    #[staticmethod]
    pub fn create_computer(name: &str, moves: Vec<usize>) -> Self
    {
        Player::builder()
            .with_name(name)
            .with_strategy(
                PredefinedMoves::from_iterable_nondiscarding(moves).unwrap()
            )
            .build()
            .into()
    }
}


#[pyclass(name="Game")]
pub struct PyGame {
    game: Option<Game<Player, Player>>
}

impl From<Game<Player, Player>> for PyGame {
    fn from(game: Game<Player, Player>) -> Self {
        // PyGame { game }
        PyGame { game: Some(game) }
    }
}

#[pymethods]
impl PyGame {
    #[rustfmt::skip]
    #[new]
    pub fn new_with_players(player1: &mut PyPlayer, player2: &mut PyPlayer) -> PyGame {
        Game::new()
            .add_player(player1.player.take().unwrap())
            .add_player(player2.player.take().unwrap())
            .into()
    }

    #[deprecated]
    #[rustfmt::skip]
    #[staticmethod]
    pub fn new_with_hardcoded_players() -> PyGame {
        Game::new()
            .add_player(
                Player::builder()
                    .human()
                    .with_name("Thomas")
                    .build()
        )
        .add_player(
            Player::builder()
                .with_name("Jay")
                .with_strategy(
                    PredefinedMoves::from_iterable_nondiscarding([5, 1, 3, 7, 9, 2, 4, 6, 8]).unwrap()
                )
                .build()
        )
        .into()
    }

    pub fn play_match(&mut self) -> PyCompletedGame {
        let game = self.game.take().unwrap();
        let completed_game = game.play_match();

        completed_game.into()
    }

}

#[pyclass(name="CompletedGame")]
pub struct PyCompletedGame {
    game: CompletedGame
}

impl From<CompletedGame> for PyCompletedGame {
    fn from(game: CompletedGame) -> Self {
        PyCompletedGame { game }
    }
}

#[pymethods]
impl PyCompletedGame {
    pub fn __repr__(&self) -> String {
        // format!("{:?}", self.game)
        format!("{:#?}", self.game)

        // Implement __repr__ from scratch for...
        // CompletedGame
        //   - Player
        //     - MonoStrategy
        //       - KeyboardStrategy
        //       - PredefinedMovesStrategy
    }

    pub fn __str__(&self) -> String {
        format!("{:}", self.game)
    }
}

#[pymodule]
mod tictactoe {
    use super::*;

    #[pymodule_export]
    use PyPlayer;

    #[pymodule_export]
    use PyGame;

    #[pymodule_export]
    use PyCompletedGame;


    #[pymodule]
    mod exception {

    }

    #[pymodule_init]
    pub fn init(m: &Bound<'_, PyModule>) -> PyResult<()> {
        pyo3_log::init();

        // Submodule setup, <https://github.com/PyO3/pyo3/discussions/5397#discussioncomment-14298706>
        let modules = PyModule::import(m.py(), "sys")?.getattr("modules")?;
        modules.set_item("tictactoe.exception", m.getattr("exception")?)?;

        Ok(())
    }
}
