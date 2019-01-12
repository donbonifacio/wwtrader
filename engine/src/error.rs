use std::fmt;

/// An enum containing all kinds of game engine errors.
#[derive(Debug, PartialEq)]
pub enum EngineError {
    InvalidEntityId(i32),
}

impl fmt::Display for EngineError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            EngineError::InvalidEntityId(id) => write!(f, "Invalid entity: {}", id),
        }
    }
}
