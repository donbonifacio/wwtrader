use actions::action::{ActionData, ActionType};
use models::coordinate::Coordinate;
use models::direction;
use models::direction::Direction;
use models::world::World;

pub fn left(entity_id: i32) -> ActionData {
    ActionData {
        entity_id: entity_id,
        action_type: ActionType::Move,
        direction: Some(direction::LEFT),
    }
}

pub fn right(entity_id: i32) -> ActionData {
    ActionData {
        entity_id: entity_id,
        action_type: ActionType::Move,
        direction: Some(direction::RIGHT),
    }
}

pub fn up(entity_id: i32) -> ActionData {
    ActionData {
        entity_id: entity_id,
        action_type: ActionType::Move,
        direction: Some(direction::UP),
    }
}

pub fn down(entity_id: i32) -> ActionData {
    ActionData {
        entity_id: entity_id,
        action_type: ActionType::Move,
        direction: Some(direction::DOWN),
    }
}

pub fn process(world: &mut World, action: ActionData) {
    if let Some(entity) = world.get_entity(action.entity_id) {
        if let Some(dir) = action.direction {
            let coord = operate(entity.coord, dir);
            let new_entity = entity.with_coordinate(coord);
            world.update_entity(new_entity);
        }
    }
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
        process(world, left);
    }

    #[test]
    fn go_left() {
        expect_position(0, 0, left, -1, 0);
        expect_position(1, 1, left, 0, 1);
    }

    #[test]
    fn go_right() {
        expect_position(0, 0, right, 1, 0);
        expect_position(1, 1, right, 2, 1);
    }

    #[test]
    fn go_down() {
        expect_position(0, 0, down, 0, 1);
        expect_position(1, 1, down, 1, 2);
    }

    #[test]
    fn go_up() {
        expect_position(0, 0, up, 0, -1);
        expect_position(1, 1, up, 1, 0);
    }

    fn expect_position(x: i8, y: i8, f: fn(i32) -> ActionData, ex: i8, ey: i8) {
        let world: &mut World = &mut World::new();
        let entity = world.register(Entity::new(0, Coordinate::new(x, y)));

        let left = f(entity.id);
        process(world, left);

        let new_entity = world.get_entity(entity.id).unwrap();
        assert_eq!(new_entity.coord.x, ex);
        assert_eq!(new_entity.coord.y, ey);
    }
}
