use actions::action::{ActionData, ActionType};
use actions::error::ActionError;
use actions::result::ActionResult;
use models::coordinate::Coordinate;
use models::direction;
use models::direction::Direction;
use models::entity::Entity;
use models::world::World;

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

fn get_entity(world: &World, entity_id: i32) -> ActionResult<Entity> {
    if let Some(entity) = world.get_entity(entity_id) {
        return Ok(entity);
    }

    Err(ActionError::InvalidEntityId(entity_id))
}

pub fn process(world: &mut World, action: ActionData) -> ActionResult<()> {
    let entity = get_entity(world, action.entity_id)?;

    if let Some(dir) = action.direction {
        let new_coord = operate(entity.coord, dir);

        is_inside_world(world, new_coord)?;
        is_position_available(world, new_coord)?;

        let new_entity = entity.with_coordinate(new_coord);
        world.update_entity(new_entity);
    }

    Ok(())
}

fn is_inside_world(world: &World, coord: Coordinate) -> ActionResult<()> {
    if !coord.is_within(world.left_edge, world.right_edge) {
        Err(ActionError::OutOfMapCoordinate(coord.x, coord.y))
    } else {
        Ok(())
    }
}

fn is_position_available(world: &World, coord: Coordinate) -> ActionResult<()> {
    match world.on_coord(coord) {
        Some(_) => Err(ActionError::PositionOccupied(coord.x, coord.y)),
        None => Ok(()),
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
    use actions::error::ActionError;
    use models::coordinate::Coordinate;
    use models::entity::Entity;

    #[test]
    fn with_invalid_entity() {
        let world: &mut World = &mut World::new();
        let left = left(1234);
        let result = process(world, left);
        assert_eq!(result.err(), Some(ActionError::InvalidEntityId(1234)));
    }

    #[test]
    fn go_left() {
        expect_position(1.0, 0.0, left, 0.0, 0.0);
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
        expect_position(0.0, 1.0, up, 0.0, 0.0);
        expect_position(1.0, 1.0, up, 1.0, 0.0);
    }

    #[test]
    fn move_to_occupied_position() {
        let world: &mut World = &mut World::new();
        let entity1 = world.register(Entity::new(0, Coordinate::new(1.0, 1.0)));
        let entity2 = world.register(Entity::new(1, Coordinate::new(1.0, 2.0)));

        let action = down(entity1.id);
        let result = process(world, action);
        assert!(result.is_err());
        assert_eq!(result.err(), Some(ActionError::PositionOccupied(1.0, 2.0)));

        let new_entity = world.get_entity(entity1.id).unwrap();
        assert!(new_entity.coord.is_at_x(1.0));
        assert!(new_entity.coord.is_at_y(1.0));

        let new_entity2 = world.get_entity(entity2.id).unwrap();
        assert!(new_entity2.coord.is_at_x(1.0));
        assert!(new_entity2.coord.is_at_y(2.0));
    }

    #[test]
    fn world_ends_error() {
        let edge = Coordinate::new(8.0, 8.0);
        let world: &mut World = &mut World::create(edge);

        expect_error(
            world,
            Coordinate::new(0.0, 0.0),
            up,
            ActionError::OutOfMapCoordinate(0.0, -1.0),
        );
        expect_error(
            world,
            Coordinate::new(0.0, 0.0),
            left,
            ActionError::OutOfMapCoordinate(-1.0, 0.0),
        );
        expect_error(
            world,
            edge,
            right,
            ActionError::OutOfMapCoordinate(edge.x + 1.0, edge.y),
        );
        expect_error(
            world,
            edge,
            down,
            ActionError::OutOfMapCoordinate(edge.x, edge.y + 1.0),
        );
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

    fn expect_error(
        world: &mut World,
        position: Coordinate,
        f: fn(i32) -> ActionData,
        error: ActionError,
    ) {
        let entity = world.register(Entity::new(0, position));

        let op = f(entity.id);
        let result = process(world, op);
        assert_eq!(result.err(), Some(error));

        let new_entity = world.get_entity(entity.id).unwrap();
        assert!(new_entity.coord.is_at_x(position.x));
        assert!(new_entity.coord.is_at_y(position.y));
    }
}
