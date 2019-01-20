use actions::action::ActionData;
use actions::error::ActionError;
use actions::result::ActionResult;
use models::entity::Entity;
use models::world::World;

pub fn get_entity(world: &World, entity_id: i32) -> ActionResult<Entity> {
    if let Some(entity) = world.get_entity(entity_id) {
        return Ok(entity);
    }

    Err(ActionError::InvalidEntityId(entity_id))
}

pub fn get_target_id(action: ActionData) -> ActionResult<i32> {
    if let Some(id) = action.target_entity_id {
        Ok(id)
    } else {
        Err(ActionError::EmptyTargetEntityId)
    }
}
