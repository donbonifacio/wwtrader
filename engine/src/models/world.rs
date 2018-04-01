
use std::collections::{HashMap};
use std::any::{Any, TypeId};

use models::entity::Entity;
use models::coordinate::Coordinate;
use actions::action::ActionData;
use actions::movement;

#[derive(Clone, Default)]
pub struct World {
    current_id: i32,
    entities: HashMap<i32, Entity>,
    actions: Vec<ActionData>
}

impl World {
    pub fn new() -> World {
        World {
            current_id: 0,
            entities: HashMap::new(),
            actions: vec![]
        }
    }

    pub fn get_entity(&self, entity_id: i32) -> Option<Entity> {
        self.entities.get(&entity_id).map(|e| e.clone())
    }

    pub fn update_entity(&mut self, entity: Entity) {
        self.entities.insert(entity.id, entity);
    }

    pub fn register(&mut self, entity: Entity) -> Entity {
        self.current_id += 1;
        let new_entity: Entity = entity.with_id(self.current_id);
        self.entities.insert(self.current_id, new_entity);
        new_entity
    }

    pub fn register_action(&mut self, action: ActionData) {
        self.actions.push(action);
    }

    pub fn has_actions(&self) -> bool {
        self.actions.len() > 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn register_entity() {
        let mut world: World = World::new();

        let entity = world.register(Entity::new(0, Coordinate::new(0, 0)));
        assert_eq!(1, entity.id);
        assert_eq!(entity.id, world.entities.get(&entity.id).unwrap().id);

        let entity2 = world.register(Entity::new(0, Coordinate::new(0, 0)));
        assert_eq!(2, entity2.id);
        assert_eq!(entity2.id, world.entities.get(&entity2.id).unwrap().id);
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
