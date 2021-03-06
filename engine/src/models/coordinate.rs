use std::fmt;

#[derive(Clone, Default, Debug, Copy)]
pub struct Coordinate {
    pub x: f32,
    pub y: f32,
}

const ERROR_MARGIN: f32 = std::f32::EPSILON;

impl fmt::Display for Coordinate {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Coord[x: {}, y: {}]", self.x, self.y)
    }
}

impl Coordinate {
    /// Returns a new `Coordinate`
    pub fn new(x: f32, y: f32) -> Coordinate {
        Coordinate { x, y }
    }

    pub fn translate(self, dx: f32, dy: f32) -> Coordinate {
        Coordinate::new(self.x + dx, self.y + dy)
    }

    pub fn is_adjacent(self, other: Coordinate) -> bool {
        let dx: f32 = self.x - other.x;
        let dy: f32 = self.y - other.y;
        dx < 2.0 && dx > -2.0 && dy < 2.0 && dy > -2.0
    }

    pub fn is_at_x(self, x: f32) -> bool {
        (self.x - x).abs() < ERROR_MARGIN
    }

    pub fn is_at_y(self, y: f32) -> bool {
        (self.y - y).abs() < ERROR_MARGIN
    }

    pub fn is_within(self, left: Coordinate, right: Coordinate) -> bool {
        !(self.x < left.x || self.y < left.y || self.x > right.x || self.y > right.y)
    }
}

impl PartialEq for Coordinate {
    fn eq(&self, other: &Coordinate) -> bool {
        self.is_at_x(other.x) && self.is_at_y(other.y)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_new() {
        let coord: Coordinate = Coordinate::new(1.0, 2.0);
        assert!(coord.is_at_x(1.0));
        assert!(coord.is_at_y(2.0));
    }

    #[test]
    fn is_at() {
        let coord: Coordinate = Coordinate::new(1.0, 1.0);
        assert!(coord.is_at_x(1.0));
        assert!(!coord.is_at_x(1.1));
        assert!(!coord.is_at_x(1.01));

        assert!(coord.is_at_y(1.0));
        assert!(!coord.is_at_y(0.09));
    }

    #[test]
    fn is_eq() {
        let coord1: Coordinate = Coordinate::new(1.0, 1.0);
        let coord2: Coordinate = Coordinate::new(1.0, 1.0);
        let coord3: Coordinate = Coordinate::new(1.0, 2.0);

        assert!(coord1 == coord2);
        assert!(coord2 != coord3);
    }

    #[test]
    fn is_within() {
        let left_edge = Coordinate::new(0.0, 0.0);
        let right_edge = Coordinate::new(8.0, 8.0);

        assert!(Coordinate::new(1.0, 1.0).is_within(left_edge, right_edge));
        assert!(Coordinate::new(4.0, 6.0).is_within(left_edge, right_edge));
        assert!(left_edge.is_within(left_edge, right_edge));
        assert!(right_edge.is_within(left_edge, right_edge));

        assert!(!Coordinate::new(-1.0, 0.0).is_within(left_edge, right_edge));
        assert!(!Coordinate::new(-9.0, 0.0).is_within(left_edge, right_edge));
        assert!(!Coordinate::new(0.0, -10.0).is_within(left_edge, right_edge));
        assert!(!Coordinate::new(4.0, 10.0).is_within(left_edge, right_edge));
    }

    #[test]
    fn is_adjacent() {
        is_adjacent_coords(0.0, 0.0, 1.0, 0.0, true);
        is_adjacent_coords(0.0, 0.0, 1.0, 1.0, true);
        is_adjacent_coords(-1.0, 0.0, 0.0, 0.0, true);

        is_adjacent_coords(-1.0, 0.0, 0.0, 5.0, false);
    }

    fn is_adjacent_coords(x1: f32, y1: f32, x2: f32, y2: f32, expected: bool) {
        let result = Coordinate::is_adjacent(Coordinate::new(x1, y1), Coordinate::new(x2, y2));
        assert!(result == expected)
    }
}
