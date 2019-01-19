use std::fmt;

/// An enum containing all kinds of action errors.
#[derive(Debug, PartialEq)]
pub enum ActionError {
    InvalidEntityId(i32),
    OutOfMapCoordinate(f32, f32),
    PositionOccupied(f32, f32),
}

impl fmt::Display for ActionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ActionError::InvalidEntityId(id) => write!(f, "Invalid entity: {}", id),
            ActionError::OutOfMapCoordinate(x, y) => {
                write!(f, "Coordinate is outside world: {},{}", x, y)
            }
            ActionError::PositionOccupied(x, y) => write!(f, "Coordinate is occupied: {},{}", x, y),
        }
    }
}
