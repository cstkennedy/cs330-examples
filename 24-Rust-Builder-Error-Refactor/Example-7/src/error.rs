use thiserror::Error;

pub trait InvariantError: std::error::Error {}

#[derive(Debug, Error)]
pub enum CostError {
    #[error("'unit cost' must be > {0}")]
    InvalidCost(f64),
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
pub enum NumericInvariantError<T> {
    #[error("{attribute} must be > {lower}")]
    LessThanBound { attribute: &'static str, lower: T },
}

impl From<CostError> for ParseRoomError {
    fn from(cost_err: CostError) -> Self {
        match cost_err {
            CostError::InvalidCost(bound) => NumericInvariantError::LessThanBound {
                attribute: "cost",
                lower: bound,
            }
            .into(),
        }
    }
}

impl From<DimensionError> for ParseRoomError {
    fn from(cost_err: DimensionError) -> Self {
        match cost_err {
            DimensionError::InvalidDimensions(bound) => NumericInvariantError::LessThanBound {
                attribute: "length and width",
                lower: bound,
            }
            .into(),
            DimensionError::InvalidLength(bound) => NumericInvariantError::LessThanBound {
                attribute: "length",
                lower: bound,
            }
            .into(),
            DimensionError::InvalidWidth(bound) => NumericInvariantError::LessThanBound {
                attribute: "width",
                lower: bound,
            }
            .into(),
        }
    }
}

#[derive(Debug, Error)]
#[error("T.B.W.")]
pub struct StringInvariantError;

#[derive(Debug, Error)]
#[error("T.B.W.")]
pub struct ParseError;

#[derive(Debug, Error)]
pub enum PlainTextParserError {
    #[error("'{delim}' missing in '{line}'")]
    MissingDelimiter { delim: char, line: String },

    #[error("'{line}' is malformed")]
    TooFewTokens { line: String },

    #[error("'{0}' is malformed")]
    MalformedLine(String),
}

impl PlainTextParserError {
    pub(crate) fn too_few_tokens(line: &str) -> Self {
        PlainTextParserError::TooFewTokens {
            line: line.to_owned(),
        }
    }

    pub(crate) fn missing_semicolon(line: &str) -> Self {
        PlainTextParserError::MissingDelimiter {
            delim: ';',
            line: line.to_owned(),
        }
    }
}

#[derive(Debug, Error)]
pub enum ParseRoomError {
    #[error("{0}")]
    PlainTextError(#[from] PlainTextParserError),

    #[error("{0}")]
    CsvError(#[from] csv::Error),

    #[error("{0}")]
    IOError(#[from] std::io::Error),

    #[error("{0:?}")]
    ParseFloatError(#[from] std::num::ParseFloatError),

    #[error("{0}")]
    FloatNumericError(#[from] NumericInvariantError<f64>),

    #[error("{0}")]
    InvariantError(#[from] HouseError),
}

#[derive(Debug, Error)]
pub enum HouseError {
    #[error("A house must have at least 1 room")]
    ZeroRooms,
}
