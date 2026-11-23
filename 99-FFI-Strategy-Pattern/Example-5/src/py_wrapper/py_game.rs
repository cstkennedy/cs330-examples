use pyo3::prelude::*;

use crate::game::{CompletedGame, Game};
use crate::player::Player;
use crate::py_wrapper::py_player::*;
use crate::strategy::*;
use crate::py_wrapper::traits::{self, DunderRepr, PyWrapper, PyWrapped};

#[pyclass(name = "Game")]
pub struct PyGame {
    game: Option<Game<Player, Player>>,
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

    pub fn __repr__(&self) -> String {
        todo!()
    }
}

impl PyWrapped for Game<Player, Player> {
    type Language = traits::language::RustForPython;
    type WrappedBy = PyGame;
}

impl DunderRepr for Game<Player, Player> {
    type Error = std::convert::Infallible; // Switch to '!' (Never type)

    fn repr_helper(&self) -> Result<String, Self::Error> {
        todo!()
    }
}

#[pyclass(name = "CompletedGame")]
pub struct PyCompletedGame {
    game: CompletedGame,
}

impl From<CompletedGame> for PyCompletedGame {
    fn from(game: CompletedGame) -> Self {
        PyCompletedGame { game }
    }
}

impl PyWrapper for PyCompletedGame {
    type WrappedType = CompletedGame;

    fn get_wrapped(&self) -> &Self::WrappedType {
        &self.game
    }
}

impl PyWrapped for CompletedGame {
    type Language = traits::language::RustForPython;
    type WrappedBy = PyCompletedGame;
}

impl DunderRepr for CompletedGame {
    type Error = std::convert::Infallible; // Switch to '!' (Never type)

    fn repr_helper(&self) -> Result<String, Self::Error> {
        match self {
            CompletedGame::Win { winner, loser } => Ok(format!(
                "CompletedGame::Win(\n  winner={},\n  loser={}\n)",
                winner.repr_helper()?,
                loser.repr_helper()?
            )),
            _ => Ok("...".to_owned()),
        }
    }
}

#[pymethods]
impl PyCompletedGame {
    pub fn __repr__(&self) -> String {
        // self.game.repr_helper().unwrap()
        self.repr_helper().unwrap()
    }

    pub fn __str__(&self) -> String {
        self.game.to_string()
    }
}
