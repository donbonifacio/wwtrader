use actions::action::{ActionData, ActionType};
use actions::common::{get_entity, get_target_id};
use actions::result::ActionResult;
use models::world::World;

pub fn direct(entity_id: i32, target_entity_id: i32) -> ActionData {
    ActionData {
        entity_id,
        target_entity_id: Some(target_entity_id),
        action_type: ActionType::Attack,
        direction: None,
    }
}

pub fn process(world: &mut World, action: ActionData) -> ActionResult<()> {
    let _attacker = get_entity(world, action.entity_id)?;

    world
        .get_entity(get_target_id(action)?)
        .map(|target| target.take_damage(1.0))
        .map(|new_target| {
            new_target.hit_points.map(|hit_points| {
                if hit_points <= 0.0 {
                    world.remove_entity(new_target);
                } else {
                    world.update_entity(new_target);
                }
            })
        });

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use actions::error::ActionError;
    use models::coordinate::Coordinate;
    use models::entity::Entity;

    #[test]
    fn with_invalid_entity() {
        let world: &mut World = &mut World::new();
        let attack = direct(1, 2);
        let result = process(world, attack);
        assert_eq!(result.err(), Some(ActionError::InvalidEntityId(1)));

        let _entity1 = world.register(Entity::new(0, Coordinate::new(1.0, 1.0)));
        let result = process(world, attack);
        assert!(result.is_ok());
    }

    #[test]
    fn with_invalid_target_id() {
        let world: &mut World = &mut World::new();
        let _entity1 = world.register(Entity::new(0, Coordinate::new(1.0, 1.0)));
        let attack = ActionData {
            target_entity_id: None,
            ..direct(1, 2)
        };
        let result = process(world, attack);
        assert_eq!(result.err(), Some(ActionError::EmptyTargetEntityId));
    }

    #[test]
    fn attack_entity() {
        let world: &mut World = &mut World::new();
        world.register(Entity::new(0, Coordinate::new(1.0, 1.0)));
        let entity2 = Entity {
            hit_points: Some(1.0),
            ..Entity::new(0, Coordinate::new(1.0, 2.0))
        };
        world.register(entity2);

        let attack = direct(1, 2);
        let result = process(world, attack);

        assert!(result.is_ok());
        assert!(world.get_entity(2).is_none());
    }

    #[test]
    fn properly_handle_multiple_attacks() {
        let world: &mut World = &mut World::new();
        world.register(Entity::new(0, Coordinate::new(1.0, 1.0)));
        let entity2 = Entity {
            hit_points: Some(1.0),
            ..Entity::new(0, Coordinate::new(1.0, 2.0))
        };
        world.register(entity2);

        let attack = direct(1, 2);
        let result = process(world, attack);
        assert!(!result.is_err());
        let result = process(world, attack);
        assert!(!result.is_err());

        assert!(world.get_entity(2).is_none());
    }
}
