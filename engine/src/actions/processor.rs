use actions::action::{ActionData, ActionType};
use actions::movement;
use engine_result::EngineResult;
use models::world::World;

pub fn process_actions(world: &mut World, actions: &[ActionData]) -> EngineResult<()> {
    for action in actions {
        process_action(world, *action)?;
    }

    Ok(())
}

fn process_action(world: &mut World, action: ActionData) -> EngineResult<()> {
    match action.action_type {
        ActionType::Move => movement::process(world, action),
    }
}
