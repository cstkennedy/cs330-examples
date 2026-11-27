use pyo3::prelude::*;

use crate::mono_strategy::MonoStrategy;
use crate::player::Player;
use crate::py_wrapper::py_strategy::*;
use crate::py_wrapper::traits::{DunderRepr, PyWrapped, PyWrapper};
use crate::strategy::{PredefinedMoves, Strategy};

impl PyWrapped for Option<Player> {
    type Language = super::traits::language::RustForPython;
    type WrappedBy = PyPlayer;
}

impl DunderRepr for Player {
    type Error = PyErr;

    fn repr_helper(&self) -> Result<String, Self::Error> {
        // Zen of Python... Flat is better than nested
        /*
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
        */
        let repr_str = match (self.is_human(), self.get_strategy()) {
            (true, _) => format!("Player.create_human(name=\"{}\")", self.get_name()),
            (false, MonoStrategy::PredefinedMoves { .. }) => {
                format!("Player.create_computer(name=\"{}\",...)", self.get_name())
            }
            (false, MonoStrategy::PyStrategy { wrapped }) => {
                format!(
                    "Player.create_custom_computer(name=\"{}\", strategy={})",
                    self.get_name(),
                    wrapped.repr_helper()?
                )
            }
            (false, MonoStrategy::Keyboard { .. }) => todo!(),
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

impl PyWrapper for PyPlayer {
    type WrappedType = Option<Player>;

    fn get_wrapped(&self) -> &Self::WrappedType {
        &self.player
    }
}

impl PyPlayer {
    #[rustfmt::skip]
    fn impl_create_computer<S>(name: &str, strategy: S) -> PyPlayer
    where
        S: Strategy + Into<MonoStrategy>,
    {
        Player::builder()
            .with_name(name)
            .with_strategy(strategy)
            .build()
            .into()
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
    pub fn create_computer(name: &str, moves: Vec<usize>) -> PyResult<Self> {
        Ok(Self::impl_create_computer(
            name,
            PredefinedMoves::from_iterable_nondiscarding(moves).unwrap(),
        ))
    }

    /*
    #[staticmethod]
    pub fn create_custom_computer(name: &str, strategy: Py<PyAny>) -> PyResult<Self> {
        Ok(Self::impl_create_computer(name, PyStrategy::new(strategy)))
    }
    */

    /*
    #[staticmethod]
    pub fn create_custom_computer(name: &str, strategy: Bound<'_, PyAny>) -> PyResult<Self> {
        Ok(Self::impl_create_computer(name, PyStrategy::new(strategy.unbind())))
    }
    */

    /*
    #[staticmethod]
    pub fn create_custom_computer(name: &str, strategy: Bound<'_, PyAny>) -> PyResult<Self> {
        let next_move_func = strategy.getattr("next_move")?;
        if !next_move_func.is_callable() {
            let class_name = strategy.get_type().name()?;
            return Err(pyo3::exceptions::PyTypeError::new_err(format!(
                "'{class_name}.next_move' is not callable"
            )));
        }

        let strategy = strategy.unbind();
        Ok(Self::impl_create_computer(name, PyStrategy::new(strategy)))
    }
    */

    #[staticmethod]
    pub fn create_custom_computer(name: &str, strategy: PyStrategy) -> PyResult<Self> {
        Ok(Self::impl_create_computer(name, strategy))
    }

    #[getter]
    pub fn get_name(&self) -> PyResult<String> {
        match &self.player {
            Some(player) => Ok(player.get_name().to_owned()),
            None => todo!()
        }
    }

    pub fn __repr__(&self) -> PyResult<String> {
        self.repr_helper()
    }
}

impl<'a, 'py> FromPyObject<'a, 'py> for PyStrategy {
    type Error = PyErr;

    fn extract(strategy: Borrowed<'a, 'py, PyAny>) -> Result<Self, Self::Error> {
        let next_move_func = strategy.getattr("next_move")?;
        if !next_move_func.is_callable() {
            let class_name = strategy.get_type().name()?;
            return Err(pyo3::exceptions::PyTypeError::new_err(format!(
                "'{class_name}.next_move' is not callable"
            )));
        }

        let strategy = strategy.to_owned().unbind();
        Ok(PyStrategy::new(strategy))
    }
}
