use thiserror::Error;

pub trait InvariantError: std::error::Error {}

#[derive(Debug, Error)]
pub enum CostError {
    #[error("'unit cost' must be > {0}")]
    InvalidCost(f64),
    // #[error("{0:?}")]
    // ParseFloatError(#[from] std::num::ParseFloatError),
}

#[derive(Debug, Error)]
pub enum DimensionError {
    #[error("'length' and 'width' must be > {0}")]
    InvalidDimensions(f64),

    #[error("'length' must be > {0}")]
    InvalidLength(f64),

    #[error("'width' must be > {0}")]
    InvalidWidth(f64),
}

#[derive(Debug, Error)]
pub enum ParseRoomError {
    #[error("'{delim}' missing in '{line}'")]
    MissingDelimiter { delim: char, line: String },

    #[error("'{line}' is malformed")]
    TooFewTokens { line: String },

    #[error("'{0}' is malformed")]
    MalformedLine(String),

    #[error("{0}")]
    IOError(#[from] std::io::Error),

    #[error("{0:?}")]
    ParseFloatError(#[from] std::num::ParseFloatError),

    #[error("{0}")]
    DimensionError(#[from] DimensionError),

    #[error("{0}")]
    CostError(#[from] CostError),
}

impl ParseRoomError {
    pub(crate) fn too_few_tokens(line: &str) -> Self {
        ParseRoomError::TooFewTokens {
            line: line.to_owned(),
        }
    }

    pub(crate) fn missing_semicolon(line: &str) -> Self {
        ParseRoomError::MissingDelimiter {
            delim: ';',
            line: line.to_owned(),
        }
    }
}

#[derive(Debug, Error)]
pub enum HouseError {
    #[error("A house must have at least 1 room")]
    ZeroRooms,
}
