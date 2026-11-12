use compact_str::CompactString;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum CreationError {
    #[error("{0:?} is not known")]
    UnknownShapeError(CompactString),

    #[error("{name:?} requires '{num_required}' dimension(s)")]
    DimensionCountError {
        name: &'static str,
        num_required: usize,
    },

    #[error("{0}")]
    MalformedLineError(CompactString),
}
