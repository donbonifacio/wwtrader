use actions::action::ActionData;
use actions::movement;
use actions::processor;
use models::world::World;

type Result<T> = std::result::Result<T, movement::MovementError>;

pub fn run(world: &mut World) -> Result<()> {
    if !world.has_actions() {
        return Ok(());
    }
    let result = run_actions(world);
    if result.is_ok() {
        world.clear_actions();
    }

    result
}

fn run_actions(world: &mut World) -> Result<()> {
    let actions = get_actions(world);
    processor::process_actions(world, &actions)
}

fn get_actions(world: &World) -> Vec<ActionData> {
    world.get_actions().to_vec()
}

#[cfg(test)]
mod tests {
    use super::*;
    use actions::movement;
    use models::coordinate::Coordinate;
    use models::entity::Entity;

    #[test]
    fn run_actions() {
        let mut world: World = World::new();
        let entity = world.register(Entity::new(0, Coordinate::new(0.0, 0.0)));

        let move_down = movement::down(entity.id);
        world.register_action(move_down);

        let move_right = movement::right(entity.id);
        world.register_action(move_right);

        assert!(run(&mut world).is_ok());

        assert!(!world.has_actions());

        let entity_option = world.get_entity(entity.id);
        assert!(entity_option.is_some());

        let entity = entity_option.unwrap();
        assert!(entity.coord.is_at_x(1.0));
        assert!(entity.coord.is_at_y(1.0));
    }
}
