use thiserror::Error;

#[derive(Error, Debug)]
pub enum CreationError {
    #[error("{0:?} is not known")]
    UnknownShapeError(String),

    #[error("{name:?} requires '{num_required}' dimension(s)")]
    DimensionCountError { name: String, num_required: usize },

    #[error("{0}")]
    MalformedLineError(String),
}
