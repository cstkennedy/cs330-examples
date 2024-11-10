use thiserror::Error;

#[derive(Debug, Error, PartialEq)]
pub enum BuildError {
    #[error("{0:?}")]
    GenericError(&'static str),
    #[error("{0:?}")]
    InvalidDim(&'static str),
    #[error("A house must have at least 1 room")]
    ZeroRooms,
}

#[derive(Debug, Error, PartialEq)]
pub struct BuildErrorWithState<B> {
    #[source]
    pub the_error: BuildError,
    pub the_builder: B,
}
