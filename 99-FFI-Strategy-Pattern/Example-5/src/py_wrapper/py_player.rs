use pyo3::prelude::*;

use crate::mono_strategy::MonoStrategy;
use crate::player::Player;
use crate::py_wrapper::py_strategy::*;
use crate::strategy::{PredefinedMoves, Strategy};

impl super::traits::DunderRepr for Player {
    type Error = std::convert::Infallible;

    fn repr_helper(&self) -> Result<String, Self::Error> {
        let repr_str = match self.is_human() {
            true => format!("Player.create_human(name=\"{}\")", self.get_name()),
            false => match self.get_strategy() {
                MonoStrategy::Keyboard { .. } => todo!(),
                MonoStrategy::PredefinedMoves { .. } => {
                    format!("Player.create_computer(name=\"{}\",...)", self.get_name())
                }
                MonoStrategy::PyStrategy { wrapped } => {
                    format!(
                        "Player.create_custom_computer(name=\"{}\", strategy={})",
                        self.get_name(),
                        wrapped.repr_helper()?
                    )
                }
            },
        };

        Ok(repr_str)
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
