use pyo3::prelude::*;

use crate::game::{CompletedGame, Game};
use crate::player::Player;
use crate::py_wrapper::py_player::*;
use crate::strategy::*;

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

#[pymethods]
impl PyCompletedGame {
    pub fn __repr__(&self) -> String {
        repr_helper_game(&self.game)
    }

    pub fn __str__(&self) -> String {
        format!("{:}", self.game)
    }
}

// Implement __repr__ from scratch for...
// CompletedGame
//   - Player
//     - MonoStrategy
//       - KeyboardStrategy
//       - PredefinedMovesStrategy
fn repr_helper_game(game: &CompletedGame) -> String {
    match game {
        CompletedGame::Win { winner, loser } => format!(
            "CompletedGame::Win(\n  winner={},\n  loser={}\n)",
            repr_helper_player(winner),
            repr_helper_player(loser)
        ),
        _ => "...".to_owned(),
    }
}
