use actions::action::{ActionData, ActionType};
use engine_result::EngineResult;
use error::EngineError;
use models::coordinate::Coordinate;
use models::direction;
use models::direction::Direction;
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

pub fn process(world: &mut World, action: ActionData) -> EngineResult<()> {
    let entity = world.get_entity(action.entity_id)?;

    if let Some(dir) = action.direction {
        let new_coord = operate(entity.coord, dir);

        is_inside_world(world, new_coord)?;
        is_position_available(world, new_coord)?;

        let new_entity = entity.with_coordinate(new_coord);
        world.update_entity(new_entity);
    }

    Ok(())
}

fn is_inside_world(world: &World, coord: Coordinate) -> EngineResult<()> {
    if coord.x < 0.0
        || coord.y < 0.0
        || coord.x >= world.size_x as f32
        || coord.y >= world.size_y as f32
    {
        return Err(EngineError::OutOfMapCoordinate(coord.x, coord.y));
    }

    Ok(())
}

fn is_position_available(world: &World, coord: Coordinate) -> EngineResult<()> {
    match world.on_coord(coord) {
        Some(_) => Err(EngineError::PositionOccupied(coord.x, coord.y)),
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
    use error::EngineError;
    use models::coordinate::Coordinate;
    use models::entity::Entity;

    #[test]
    fn with_invalid_entity() {
        let world: &mut World = &mut World::new();
        let left = left(1234);
        let result = process(world, left);
        assert_eq!(result.err(), Some(EngineError::InvalidEntityId(1234)));
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
        assert_eq!(result.err(), Some(EngineError::PositionOccupied(1.0, 2.0)));

        let new_entity = world.get_entity(entity1.id).unwrap();
        assert!(new_entity.coord.is_at_x(1.0));
        assert!(new_entity.coord.is_at_y(1.0));

        let new_entity2 = world.get_entity(entity2.id).unwrap();
        assert!(new_entity2.coord.is_at_x(1.0));
        assert!(new_entity2.coord.is_at_y(2.0));
    }

    #[test]
    fn world_ends_error() {
        const SIZE_X: i8 = 8;
        const SIZE_Y: i8 = 8;
        let world: &mut World = &mut World::create(SIZE_X as usize, SIZE_Y as usize);

        expect_error(
            world,
            0.0,
            0.0,
            up,
            EngineError::OutOfMapCoordinate(0.0, -1.0),
        );
        expect_error(
            world,
            0.0,
            0.0,
            left,
            EngineError::OutOfMapCoordinate(-1.0, 0.0),
        );
        expect_error(
            world,
            f32::from(SIZE_X),
            f32::from(SIZE_Y),
            right,
            EngineError::OutOfMapCoordinate((SIZE_X + 1).into(), SIZE_Y.into()),
        );
        expect_error(
            world,
            f32::from(SIZE_X),
            f32::from(SIZE_Y),
            down,
            EngineError::OutOfMapCoordinate(SIZE_X.into(), (SIZE_Y + 1).into()),
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
        x: f32,
        y: f32,
        f: fn(i32) -> ActionData,
        error: EngineError,
    ) {
        let entity = world.register(Entity::new(0, Coordinate::new(x, y)));

        let op = f(entity.id);
        let result = process(world, op);
        assert!(result.is_err());
        assert_eq!(result.err(), Some(error));

        let new_entity = world.get_entity(entity.id).unwrap();
        assert!(new_entity.coord.is_at_x(x));
        assert!(new_entity.coord.is_at_y(y));
    }
}
