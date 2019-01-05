use models::coordinate::Coordinate;

#[derive(Clone, Debug, Copy, PartialEq)]
pub enum EntityType {
    Player(i8),
    Enemy(char),
    Obstacle(char),
    Hole(char),
}

#[derive(Clone, Debug, Copy)]
pub struct Entity {
    pub id: i32,
    pub coord: Coordinate,
    pub entity_type: EntityType, // behaviour
                                 // type
}

impl Default for Entity {
    fn default() -> Entity {
        Entity {
            id: 0,
            coord: Coordinate::new(0, 0),
            entity_type: EntityType::Player(1),
        }
    }
}

impl Entity {
    pub fn new(id: i32, coord: Coordinate) -> Entity {
        Entity {
            id: id,
            coord: coord,
            entity_type: EntityType::Enemy('?'),
        }
    }

    pub fn with_coordinate(&self, coord: Coordinate) -> Entity {
        //Entity { id: self.id, coord: coord, entity_type: self.entity_type }
        Entity {
            coord: coord,
            ..*self
        }
    }

    pub fn with_id(&self, new_id: i32) -> Entity {
        Entity {
            id: new_id,
            ..*self
        }
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
