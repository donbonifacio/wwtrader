
use models::entity::{Entity, EntityType};
use models::coordinate::Coordinate;

pub fn create_at(number: i8, coord: Coordinate) -> Entity {
    Entity {
        entity_type: EntityType::Player(number),
        coord: coord,
        ..Default::default()
    }
}
