
use models::coordinate::Coordinate;

#[derive(Clone, Default, Debug, Copy)]
pub struct Entity {
    pub id: i32,
    pub coord: Coordinate
}

impl Entity {
    pub fn new(id: i32, coord: Coordinate) -> Entity {
        Entity { id: id, coord: coord }
    }

    pub fn with_coordinate(&self, coord: Coordinate) -> Entity {
        Entity { id: self.id, coord: coord }
    }

    pub fn with_id(self, new_id: i32) -> Entity {
        Entity { id: new_id, coord: self.coord }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_new() {
        let entity = Entity::new(1, Coordinate::new(2, 3));
        assert_eq!(1, entity.id);
        assert_eq!(2, entity.coord.x);
        assert_eq!(3, entity.coord.y);
    }
}
