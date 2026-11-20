use pyo3::prelude::*;

use crate::mono_strategy::MonoStrategy;
use crate::player::Player;
use crate::py_wrapper::py_strategy::*;
use crate::strategy::{PredefinedMoves, Strategy};

pub(crate) fn repr_helper_player(player: &Player) -> String {
    match player.is_human() {
        true => format!("Player.create_human(name=\"{}\")", player.get_name()),
        false => match player.get_strategy() {
            MonoStrategy::Keyboard { .. } => todo!(),
            MonoStrategy::PredefinedMoves { .. } => {
                format!("Player.create_computer(name=\"{}\",...)", player.get_name())
            }
            MonoStrategy::PyStrategy { wrapped } => {
                format!(
                    "Player.create_custom_computer(name=\"{}\", strategy={})",
                    player.get_name(),
                    repr_helper_py_strategy(&wrapped)
                )
            }
        },
    }
}

#[pyclass(name = "Player")]
pub struct PyPlayer {
    pub(crate) player: Option<Player>,
}

impl From<Player> for PyPlayer {
    fn from(player: Player) -> PyPlayer {
        PyPlayer {
            player: Some(player),
        }
    }
}

impl PyPlayer {
    #[rustfmt::skip]
    fn impl_create_computer<S>(name: &str, strategy: S) -> Player
    where
        S: Strategy + Into<MonoStrategy>,
    {
        Player::builder()
            .with_name(name)
            .with_strategy(strategy)
            .build()
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

    #[staticmethod]
    pub fn create_computer(name: &str, moves: Vec<usize>) -> Self {
        Self::impl_create_computer(
            name,
            PredefinedMoves::from_iterable_nondiscarding(moves).unwrap(),
        )
        .into()
    }

    #[rustfmt::skip]
    #[staticmethod]
    pub fn create_custom_computer(name: &str, strategy: Py<PyAny>) -> Self
    {
        Player::builder()
            .with_name(name)
            .with_strategy(
                PyStrategy::new(strategy)
            )
            .build()
            .into()
    }
}
