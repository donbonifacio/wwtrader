
use models::entity::{Entity, EntityType};
use models::coordinate::Coordinate;

pub fn create_at(coord: Coordinate) -> Entity {
    Entity {
        entity_type: EntityType::Player(1),
        coord: coord,
        ..Default::default()
    }
}
