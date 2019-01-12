use actions::action::{ActionData, ActionType};
use actions::movement;
use models::world::World;

type Result<T> = std::result::Result<T, movement::MovementError>;

pub fn process_actions(world: &mut World, actions: &[ActionData]) -> Result<()> {
    for action in actions {
        process_action(world, *action)?;
    }

    Ok(())
}

fn process_action(world: &mut World, action: ActionData) -> Result<()> {
    match action.action_type {
        ActionType::Move => movement::process(world, action),
    }
}
