use models::coordinate::Coordinate;
use models::entity::{Entity, EntityType};

pub fn create_at(coord: Coordinate) -> Entity {
    Entity {
        entity_type: EntityType::Hole('~'),
        coord,
        ..Default::default()
    }
}
