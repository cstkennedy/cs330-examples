use pyo3::prelude::*;

use crate::mono_strategy::MonoStrategy;
use crate::player::Player;
use crate::py_wrapper::py_strategy::*;
use crate::strategy::{PredefinedMoves, Strategy};
use crate::py_wrapper::traits::{DunderRepr, PyWrapper, PyWrapped};


impl PyWrapped for Option<Player> {
    type Language = super::traits::language::RustForPython;
    type WrappedBy = PyPlayer;
}

impl DunderRepr for Player {
    type Error = std::convert::Infallible;

    fn repr_helper(&self) -> Result<String, Self::Error> {
        // Zen of Python... Flat is better than nested
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

// Handled by Option<T> implementation
/*
impl super::traits::DunderRepr for Option<Player> {
    type Error = std::convert::Infallible;

    type WrappedLanguage = super::traits::language::RustForPython;

    fn repr_helper(&self) -> Result<String, Self::Error> {
        match self {
            Some(player) => player.repr_helper(),
            None => Ok("...".to_string())
        }
    }
}
*/

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

impl PyWrapper for PyPlayer {
    type WrappedType = Option<Player>;

    fn get_wrapped(&self) -> &Self::WrappedType {
        &self.player
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

    fn __repr__(&self) -> String {
        self.repr_helper().unwrap()
    }
}
