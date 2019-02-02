use std::collections::HashMap;

use actions::action::ActionData;
use controllers::player::{PlayerController, PlayerInput};
use models::coordinate::Coordinate;
use models::entity::{Entity, EntityType};

#[derive(Clone)]
pub struct World {
    current_id: i32,
    pub left_edge: Coordinate,
    pub right_edge: Coordinate,
    pub entities: HashMap<i32, Entity>,
    pub player_controllers: Vec<PlayerController>,
    pub player_inputs: HashMap<i32, PlayerInput>,
    actions: Vec<ActionData>,
}

impl Default for World {
    fn default() -> World {
        World {
            current_id: 0,
            left_edge: Coordinate::new(0.0, 0.0),
            right_edge: Coordinate::new(8.0, 4.0),
            entities: HashMap::new(),
            player_controllers: vec![],
            player_inputs: HashMap::new(),
            actions: vec![],
        }
    }
}

impl World {
    pub fn new() -> World {
        World {
            current_id: 0,
            ..Default::default()
        }
    }

    pub fn create(edge: Coordinate) -> World {
        World {
            right_edge: edge,
            ..Default::default()
        }
    }

    pub fn register_player_input(&mut self, entity_id: i32, input: PlayerInput) {
        self.player_inputs.insert(entity_id, input);
    }

    pub fn clear_player_inputs(&mut self) {
        self.player_inputs.clear();
    }

    pub fn get_entity(&self, entity_id: i32) -> Option<Entity> {
        self.entities.get(&entity_id).cloned()
    }

    pub fn on_coord(&self, coord: Coordinate) -> Option<&Entity> {
        self.entities.values().find(|entity| entity.coord == coord)
    }

    pub fn update_entity(&mut self, entity: Entity) {
        self.entities.insert(entity.id, entity);
    }

    pub fn remove_entity(&mut self, entity: Entity) {
        self.entities.remove(&entity.id);
    }

    pub fn register(&mut self, entity: Entity) -> Entity {
        self.current_id += 1;
        let new_entity: Entity = entity.with_id(self.current_id);
        self.entities.insert(self.current_id, new_entity);
        new_entity
    }

    pub fn get_player(&self, player_number: i8) -> Option<Entity> {
        self.entities
            .values()
            .find(|&entity| match entity.entity_type {
                EntityType::Player(number) => number == player_number,
                _ => false,
            })
            .cloned()
    }

    pub fn register_player(&mut self, controller: PlayerController) {
        self.player_controllers.push(controller);
    }

    pub fn register_action(&mut self, action: ActionData) {
        self.actions.push(action);
    }

    pub fn has_actions(&self) -> bool {
        !self.actions.is_empty()
    }

    pub fn clear_actions(&mut self) {
        self.actions.clear()
    }

    pub fn get_actions(&self) -> &Vec<ActionData> {
        &self.actions
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use actions::movement;
    use models::coordinate::Coordinate;

    #[test]
    fn register_entity() {
        let mut world: World = World::new();

        let entity = world.register(Entity::new(0, Coordinate::new(0.0, 0.0)));
        assert_eq!(1, entity.id);
        assert_eq!(entity.id, world.get_entity(entity.id).unwrap().id);

        let entity2 = world.register(Entity::new(0, Coordinate::new(0.0, 0.0)));
        assert_eq!(2, entity2.id);
        assert_eq!(entity2.id, world.entities[&entity2.id].id);
    }

    #[test]
    fn register_action() {
        let mut world: World = World::new();

        assert!(!world.has_actions());

        let action = movement::down(123);
        world.register_action(action);
        assert!(world.has_actions());
    }
}
