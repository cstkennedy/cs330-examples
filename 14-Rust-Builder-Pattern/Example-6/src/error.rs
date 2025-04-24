use thiserror::Error;

#[derive(Debug, Error)]
pub enum ParseError {
    #[error("Missing '{delim}' in '{line}'")]
    MissingDelimiter { delim: String, line: String },

    #[error("'{line}' is malformed - only '{num_tokens} tokens'")]
    TooFewTokens { num_tokens: usize, line: String },

    #[error("'{0}' is malformed")]
    MalformedLine(String),

    #[error("{0}")]
    IOError(#[from] std::io::Error),
}

#[derive(Debug, Error)]
pub enum RoomError {
    #[error("'length' and 'width' must be > {0}")]
    InvalidDimensions(f64),

    #[error("'length' must be > {0}")]
    InvalidLength(f64),

    #[error("'width' must be > {0}")]
    InvalidWidth(f64),

    #[error("'unit cost' must be > {0}")]
    InvalidCost(f64),

    #[error("{0:?}")]
    ParseError(#[from] ParseError),
}

#[derive(Debug, Error)]
pub enum HouseError {
    #[error("A house must have at least 1 room")]
    ZeroRooms,
}

#[derive(Debug, Error)]
pub struct ErrorWithState<E: std::error::Error, S> {
    #[source]
    pub the_error: E,
    pub the_state: S,
}

#[derive(Debug, Error)]
pub struct BuildErrorWithState<E: std::error::Error, B> {
    #[source]
    pub the_error: E,
    pub the_builder: B,
}

impl<E, S> From<ErrorWithState<E, S>> for BuildErrorWithState<E, S>
where
    E: std::error::Error,
{
    fn from(source: ErrorWithState<E, S>) -> Self {
        BuildErrorWithState {
            the_error: source.the_error,
            the_builder: source.the_state,
        }
    }
}

pub type RoomErrorWithState<S> = BuildErrorWithState<RoomError, S>;
pub type HouseErrorWithState<S> = BuildErrorWithState<HouseError, S>;

impl<S> From<RoomErrorWithState<S>> for RoomError {
    fn from(source_with_state: RoomErrorWithState<S>) -> Self {
        source_with_state.the_error
    }
}

impl<S> From<HouseErrorWithState<S>> for HouseError {
    fn from(source_with_state: HouseErrorWithState<S>) -> Self {
        source_with_state.the_error
    }
}
