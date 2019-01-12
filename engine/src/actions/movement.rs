use actions::action::{ActionData, ActionType};
use models::coordinate::Coordinate;
use models::direction;
use models::direction::Direction;
use models::world::World;
use std::fmt;

pub struct MovementError {}

impl fmt::Display for MovementError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Movement error")
    }
}

type Result<T> = std::result::Result<T, MovementError>;

pub fn left(entity_id: i32) -> ActionData {
    ActionData {
        entity_id,
        action_type: ActionType::Move,
        direction: Some(direction::LEFT),
    }
}

pub fn right(entity_id: i32) -> ActionData {
    ActionData {
        entity_id,
        action_type: ActionType::Move,
        direction: Some(direction::RIGHT),
    }
}

pub fn up(entity_id: i32) -> ActionData {
    ActionData {
        entity_id,
        action_type: ActionType::Move,
        direction: Some(direction::UP),
    }
}

pub fn down(entity_id: i32) -> ActionData {
    ActionData {
        entity_id,
        action_type: ActionType::Move,
        direction: Some(direction::DOWN),
    }
}

pub fn process(world: &mut World, action: ActionData) -> Result<()> {
    if let Some(entity) = world.get_entity(action.entity_id) {
        if let Some(dir) = action.direction {
            let coord = operate(entity.coord, dir);
            let new_entity = entity.with_coordinate(coord);
            world.update_entity(new_entity);

            return Ok(());
        }
    }

    // TODO: Proper error
    Err(MovementError {})
}

pub fn operate(coord: Coordinate, direction: Direction) -> Coordinate {
    Coordinate {
        x: coord.x + direction.dx,
        y: coord.y + direction.dy,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use models::coordinate::Coordinate;
    use models::entity::Entity;

    #[test]
    fn with_invalid_entity() {
        let world: &mut World = &mut World::new();
        let left = left(1234);
        assert!(process(world, left).is_err());
    }

    #[test]
    fn go_left() {
        expect_position(0.0, 0.0, left, -1.0, 0.0);
        expect_position(1.0, 1.0, left, 0.0, 1.0);
    }

    #[test]
    fn go_right() {
        expect_position(0.0, 0.0, right, 1.0, 0.0);
        expect_position(1.0, 1.0, right, 2.0, 1.0);
    }

    #[test]
    fn go_down() {
        expect_position(0.0, 0.0, down, 0.0, 1.0);
        expect_position(1.0, 1.0, down, 1.0, 2.0);
    }

    #[test]
    fn go_up() {
        expect_position(0.0, 0.0, up, 0.0, -1.0);
        expect_position(1.0, 1.0, up, 1.0, 0.0);
    }

    fn expect_position(x: f32, y: f32, f: fn(i32) -> ActionData, ex: f32, ey: f32) {
        let world: &mut World = &mut World::new();
        let entity = world.register(Entity::new(0, Coordinate::new(x, y)));

        let left = f(entity.id);
        assert!(process(world, left).is_ok());

        let new_entity = world.get_entity(entity.id).unwrap();
        assert!(new_entity.coord.is_at_x(ex));
        assert!(new_entity.coord.is_at_y(ey));
    }
}
