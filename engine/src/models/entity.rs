use models::coordinate::Coordinate;
use std::fmt;

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
    pub entity_type: EntityType,
    pub hit_points: Option<f32>,
}

impl Default for Entity {
    fn default() -> Entity {
        Entity {
            id: 0,
            coord: Coordinate::new(0.0, 0.0),
            entity_type: EntityType::Player(1),
            hit_points: None,
        }
    }
}

impl Entity {
    pub fn new(id: i32, coord: Coordinate) -> Entity {
        Entity {
            id,
            coord,
            entity_type: EntityType::Enemy('?'),
            hit_points: None,
        }
    }

    pub fn with_coordinate(&self, coord: Coordinate) -> Entity {
        Entity { coord, ..*self }
    }

    pub fn take_damage(&self, damage: f32) -> Entity {
        self.hit_points.map_or(*self, |hit_points| Entity {
            hit_points: Some(hit_points - damage),
            ..*self
        })
    }

    pub fn with_id(&self, new_id: i32) -> Entity {
        Entity {
            id: new_id,
            ..*self
        }
    }
}

impl fmt::Display for Entity {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Entity[id: {}, type: {:?}, coord: {}, hit_points: {:?}]",
            self.id, self.entity_type, self.coord, self.hit_points
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_new() {
        let entity = Entity::new(1, Coordinate::new(2.0, 3.0));
        assert_eq!(1, entity.id);
        assert!(entity.coord.is_at_x(2.0));
        assert!(entity.coord.is_at_y(3.0));
    }
}
