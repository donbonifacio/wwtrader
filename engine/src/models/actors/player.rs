use models::coordinate::Coordinate;
use models::entity::{Entity, EntityType};

pub fn create_at(number: i8, coord: Coordinate) -> Entity {
    Entity {
        entity_type: EntityType::Player(number),
        coord,
        ..Default::default()
    }
}
