use thiserror::Error;


#[derive(Debug, Error)]
pub enum CostError {
    #[error("'unit cost' must be > {0}")]
    InvalidCost(f64),

    #[error("{0:?}")]
    ParseFloatError(#[from] std::num::ParseFloatError),
}

#[derive(Debug, Error)]
pub enum FlooringError {
    #[error("{0}")]
    CostError(#[from] CostError),
}

#[derive(Debug, Error)]
pub enum DimensionError {
    #[error("'length' and 'width' must be > {0}")]
    InvalidDimensions(f64),

    #[error("'length' must be > {0}")]
    InvalidLength(f64),

    #[error("'width' must be > {0}")]
    InvalidWidth(f64),

    #[error("{0:?}")]
    ParseFloatError(#[from] std::num::ParseFloatError),
}

// TODO: Remove
#[derive(Debug, Error)]
pub enum RoomError {
    #[error("{0}")]
    DimensionError(#[from] DimensionError),

    #[error("{0}")]
    FlooringError(#[from] FlooringError),
}

impl From<CostError> for RoomError {
    fn from(cost_error: CostError) -> Self {
        RoomError::from(FlooringError::from(cost_error))
    }
}

#[derive(Debug, Error)]
pub enum ParseRoomError {
    #[error("'{delim}' missing in '{line}'")]
    MissingDelimiter { delim: String, line: String },

    #[error("'{line}' is malformed - only '{num_tokens}' token(s)")]
    TooFewTokens { num_tokens: usize, line: String },

    #[error("'{0}' is malformed")]
    MalformedLine(String),

    #[error("{0}")]
    IOError(#[from] std::io::Error),

    #[error("{0}")]
    DimensionError(#[from] DimensionError),

    #[error("{0}")]
    FlooringError(#[from] FlooringError),
}


impl From<CostError> for ParseRoomError {
    fn from(cost_error: CostError) -> Self {
        ParseRoomError::from(FlooringError::from(cost_error))
    }
}

#[derive(Debug, Error)]
pub enum HouseError {
    #[error("{0}")]
    RoomError(#[from] RoomError),

    #[error("A house must have at least 1 room")]
    ZeroRooms,
}

impl From<CostError> for HouseError {
    fn from(cost_error: CostError) -> Self {
        HouseError::from(RoomError::from(cost_error))
    }
}

#[derive(Debug, Error)]
pub struct BuildErrorWithState<E: std::error::Error, B> {
    #[source]
    pub the_error: E,
    pub the_builder: B,
}

pub type HouseErrorWithState<S> = BuildErrorWithState<HouseError, S>;

impl<S> From<HouseErrorWithState<S>> for HouseError {
    fn from(source_with_state: HouseErrorWithState<S>) -> Self {
        source_with_state.the_error
    }
}
