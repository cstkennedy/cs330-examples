use pyo3::prelude::*;

pub mod board;
pub mod error;
pub mod game;
pub mod mono_strategy;
pub mod player;
pub mod py_wrapper;
pub mod referee;
pub mod strategy;

pub mod prelude {
    pub use crate::board::Position;
    pub use crate::game::Game;
    pub use crate::player::Player;
    pub use crate::strategy::*;
}

#[pymodule]
mod tictactoe {
    use pyo3::prelude::*;

    use crate::py_wrapper::prelude::*;

    #[pymodule_export]
    use PyPlayer;

    #[pymodule_export]
    use PyGame;

    #[pymodule_export]
    use PyCompletedGame;

    #[pymodule]
    mod exception {}

    #[pymodule_init]
    pub fn init(m: &Bound<'_, PyModule>) -> PyResult<()> {
        pyo3_log::init();

        // Submodule setup, <https://github.com/PyO3/pyo3/discussions/5397#discussioncomment-14298706>
        let modules = PyModule::import(m.py(), "sys")?.getattr("modules")?;
        modules.set_item("tictactoe.exception", m.getattr("exception")?)?;

        Ok(())
    }
}
