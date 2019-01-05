use actions::action::{ActionData, ActionType};
use actions::movement;
use models::world::World;

pub fn process_actions(world: &mut World, actions: &Vec<ActionData>) {
    for action in actions {
        process_action(world, action);
    }
}

fn process_action(world: &mut World, action: &ActionData) {
    match action.action_type {
        ActionType::Move => movement::process(world, *action),
    }
}
