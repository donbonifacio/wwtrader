use models::world::World;
use models::direction::Direction;

#[derive(Clone, Debug, Copy)]
pub enum ActionType {
    Move,
    Sleep
}

#[derive(Clone, Debug, Copy)]
pub struct ActionData {
    pub entity_id: i32,
    pub action_type: ActionType,
    pub direction: Option<Direction>
}
