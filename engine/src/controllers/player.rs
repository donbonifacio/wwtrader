use actions;
use controllers::controller::Controller;
use models::direction;
use models::direction::Direction;
use models::world::World;

#[derive(Clone, Default, Debug, Copy)]
pub struct PlayerController {
    entity_id: i32,
}

#[derive(Clone, Default, Debug, Copy)]
pub struct PlayerInput {
    direction: Direction,
}

impl PlayerInput {
    pub fn new(direction: Direction) -> PlayerInput {
        PlayerInput { direction }
    }
}

impl PlayerController {
    pub fn new(entity_id: i32) -> PlayerController {
        PlayerController { entity_id }
    }

    pub fn run(self, world: &mut World, input: PlayerInput) {
        let action = if input.direction == direction::LEFT {
            actions::movement::left(self.entity_id)
        } else if input.direction == direction::RIGHT {
            actions::movement::right(self.entity_id)
        } else if input.direction == direction::UP {
            actions::movement::up(self.entity_id)
        } else {
            actions::movement::down(self.entity_id)
        };

        world.register_action(action);
    }
}

impl Controller for PlayerController {}
