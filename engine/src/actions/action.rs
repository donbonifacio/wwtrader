use models::direction::Direction;

#[derive(Clone, Debug, Copy)]
pub enum ActionType {
    Move,
    Attack,
}

#[derive(Clone, Debug, Copy)]
pub struct ActionData {
    pub entity_id: i32,
    pub target_entity_id: Option<i32>,
    pub action_type: ActionType,
    pub direction: Option<Direction>,
}

impl Default for ActionData {
    fn default() -> ActionData {
        ActionData {
            entity_id: 0,
            target_entity_id: None,
            action_type: ActionType::Move,
            direction: None,
        }
    }
}
