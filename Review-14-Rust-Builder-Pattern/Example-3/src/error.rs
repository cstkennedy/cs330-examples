use std::fmt;
use std::fmt::Display; //,Formatter,Result};

#[derive(Debug, PartialEq)]
pub enum BuildError {
    GenericError(&'static str),
    InvalidDim(&'static str),
    ZeroRooms,
}

/*
impl From<std::io::Error> for ParseError {
    fn from(err: std::io::Error) -> Self {
        ParseError::GenericError(err)
    }
}
*/

impl Display for BuildError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            BuildError::GenericError(description) => {
                write!(f, "{:?}", description)
            }
            BuildError::ZeroRooms => {
                write!(f, "A house must have at least 1 room")
            }
            BuildError::InvalidDim(description) => {
                write!(f, "{:?}", description)
            }
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct BuildErrorWithState<B> {
    pub the_error: BuildError,
    pub the_builder: B,
}
