use std::fmt;

/// An enum containing all kinds of game engine errors.
#[derive(Debug, PartialEq)]
pub enum EngineError {
    InvalidEntityId(i32),
    OutOfMapCoordinate(f32, f32),
    PositionOccupied(f32, f32),
}

impl fmt::Display for EngineError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            EngineError::InvalidEntityId(id) => write!(f, "Invalid entity: {}", id),
            EngineError::OutOfMapCoordinate(x, y) => {
                write!(f, "Coordinate is outside world: {},{}", x, y)
            }
            EngineError::PositionOccupied(x, y) => write!(f, "Coordinate is occupied: {},{}", x, y),
        }
    }
}
