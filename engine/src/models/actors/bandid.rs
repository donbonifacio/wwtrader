use models::coordinate::Coordinate;
use models::entity::{Entity, EntityType};

pub fn create_at(coord: Coordinate) -> Entity {
    Entity {
        entity_type: EntityType::Enemy('B'),
        coord,
        hit_points: Some(1.0),
        ..Default::default()
    }
}
