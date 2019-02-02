use actions::action::{ActionData, ActionType};
use actions::attack;
use actions::movement;
use actions::result::ActionResult;
use models::world::World;

pub fn process_actions(world: &mut World, actions: &[ActionData]) -> ActionResult<()> {
    for action in actions {
        process_action(world, *action)?;
    }

    Ok(())
}

fn process_action(world: &mut World, action: ActionData) -> ActionResult<()> {
    match action.action_type {
        ActionType::Move => movement::process(world, action),
        ActionType::Attack => attack::process(world, action),
    }
}
