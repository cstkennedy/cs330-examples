use crate::error::{MoveError};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Move(usize);

impl std::ops::Deref for Move {
    type Target = usize;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl TryFrom<usize> for Move {
    type Error = MoveError;

    fn try_from(val: usize) -> Result<Move, Self::Error> {
        if val < 1 || val > 9 {
            return Err(MoveError::ValueError(val));
        }

        Ok(Move(val))
    }
}

