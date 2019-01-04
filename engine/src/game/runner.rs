
use models::world::World;
use actions::action::ActionData;
use models::coordinate::Coordinate;
use actions::movement;
use actions::processor;
use models::entity::Entity;

pub fn run(world: &mut World) {
    if !world.has_actions() {
        return;
    }
    run_actions(world);
    world.clear_actions();
}

fn run_actions(world: &mut World) {
    let actions = get_actions(world);
    processor::process_actions(world, &actions)
}

fn get_actions(world: &mut World) -> Vec<ActionData> {
    // TODO: remove clone
    world.get_actions().clone()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_actions() {
        let mut world: World = World::new();
        let entity = world.register(Entity::new(0, Coordinate::new(0, 0)));

        let move_down = movement::down(entity.id);
        world.register_action(move_down);

        let move_right = movement::right(entity.id);
        world.register_action(move_right);

        run(& mut world);

        assert!(!world.has_actions());

        let entity_option = world.get_entity(entity.id);
        assert!(entity_option.is_some());

        let entity = entity_option.unwrap();
        assert_eq!(1, entity.coord.x);
        assert_eq!(1, entity.coord.y);

    }
}