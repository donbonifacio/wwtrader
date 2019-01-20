use actions;
use actions::ActionData;
use controllers::controller::Controller;
use models::direction;
use models::direction::Direction;
use models::entity::{Entity, EntityType};
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
        let action = self.resolve_action(world, input);
        world.register_action(action)
    }

    fn resolve_action(self, world: &World, input: PlayerInput) -> ActionData {
        if let Some(target) = self.get_target(world, input) {
            actions::attack::direct(self.entity_id, target.id)
        } else {
            self.get_movement_action(input)
        }
    }

    fn get_movement_action(self, input: PlayerInput) -> ActionData {
        if input.direction == direction::LEFT {
            actions::movement::left(self.entity_id)
        } else if input.direction == direction::RIGHT {
            actions::movement::right(self.entity_id)
        } else if input.direction == direction::UP {
            actions::movement::up(self.entity_id)
        } else {
            actions::movement::down(self.entity_id)
        }
    }

    fn get_target(self, world: &World, input: PlayerInput) -> Option<Entity> {
        let range: i32 = 9;
        if let Some(player) = world.get_entity(self.entity_id) {
            for n in 1..=range {
                let n: f32 = n as f32;

                let position = player
                    .coord
                    .translate(input.direction.dx * n, input.direction.dy * n);

                if let Some(entity) = world.on_coord(position) {
                    match entity.entity_type {
                        EntityType::Obstacle(_) => return None,
                        EntityType::Enemy(_) => return Some(*entity),
                        EntityType::Player(_) => return Some(*entity),
                        _ => (),
                    }
                }
            }
        }

        None
    }
}

impl Controller for PlayerController {}
