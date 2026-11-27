use pyo3::prelude::*;

use crate::game::{CompletedGame, Game};
use crate::player::Player;
use crate::py_wrapper::py_player::*;
use crate::py_wrapper::traits::{self, DunderRepr, PyWrapped, PyWrapper};
use crate::strategy::*;

#[pyclass(name = "Game")]
pub struct PyGame {
    game: Option<Game<Player, Player>>,
}

impl From<Game<Player, Player>> for PyGame {
    fn from(game: Game<Player, Player>) -> Self {
        PyGame { game: Some(game) }
    }
}

impl<'a, 'py> FromPyObject<'a, 'py> for Player {
    type Error = PyErr;

    fn extract(obj: Borrowed<'a, 'py, PyAny>) -> Result<Self, Self::Error> {
        let mut py_player: PyRefMut<'_, PyPlayer> = obj.extract()?;

        match py_player.player.take() {
            None => Err(pyo3::exceptions::PyValueError::new_err(
                "Player has already been exhausted"
            )),
            Some(player) => Ok(player)
        }
    }
}

#[pymethods]
impl PyGame {
    /*
    #[rustfmt::skip]
    #[new]
    pub fn new_with_players(player1: &mut PyPlayer, player2: &mut PyPlayer) -> PyResult<PyGame> {
        let game = Game::new()
            .add_player(player1.player.take().unwrap())
            .add_player(player2.player.take().unwrap())
            .into();

        Ok(game)
    }
    */
    #[rustfmt::skip]
    #[new]
    pub fn new_with_players(player1: Player, player2: Player) -> PyGame {
        let game = Game::new()
            .add_player(player1)
            .add_player(player2)
            .into();

        game
    }

    pub fn play_match(&mut self) -> PyResult<PyCompletedGame> {
        let game = self.game.take().unwrap();
        let completed_game = game.play_match();

        Ok(completed_game.into())
    }

    pub fn __repr__(&self) -> PyResult<String> {
        todo!()
    }
}

impl PyWrapped for Game<Player, Player> {
    type Language = traits::language::RustForPython;
    type WrappedBy = PyGame;
}

impl DunderRepr for Game<Player, Player> {
    type Error = PyErr;

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
    type Error = PyErr;

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
    pub fn __repr__(&self) -> PyResult<String> {
        self.repr_helper()
    }

    pub fn __str__(&self) -> String {
        self.game.to_string()
    }
}
