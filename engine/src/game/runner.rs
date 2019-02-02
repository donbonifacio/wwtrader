use actions::action::ActionData;
use actions::processor;
use actions::result::ActionResult;
use controllers::player::PlayerController;
use models::world::World;

pub fn run(world: &mut World) -> ActionResult<()> {
    process_player_input(world);

    if !world.has_actions() {
        return Ok(());
    }

    let result = run_actions(world);
    world.clear_actions();

    world.clear_player_inputs();

    result
}

fn process_player_input(world: &mut World) {
    // TODO: remove clone
    for (entity_id, input) in world.player_inputs.clone().iter() {
        if let Some(controller) = get_controller(world, *entity_id) {
            controller.run(world, *input);
        };
    }
}

fn get_controller(world: &World, entity_id: i32) -> Option<PlayerController> {
    // TODO: maybe index player controllers in the world?
    for controller in world.player_controllers.iter() {
        if controller.entity_id == entity_id {
            return Some(*controller);
        }
    }

    None
}

fn run_actions(world: &mut World) -> ActionResult<()> {
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
