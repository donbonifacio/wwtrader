

#[derive(Clone, Default, Debug, Copy)]
pub struct Coordinate {
    pub x: i8,
    pub y: i8
}

impl Coordinate {
    /// Returns a new `Coordinate`
    pub fn new(x: i8, y: i8) -> Coordinate {
        Coordinate { x: x, y: y }
    }

    pub fn is_adjacent(self, other: Coordinate) -> bool {
        let dx: i8 = self.x - other.x;
        let dy: i8 = self.y - other.y;
        dx < 2 && dx > -2 && dy < 2 && dy > -2
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_new() {
        let coord: Coordinate = Coordinate::new(1, 2);
        assert_eq!(1, coord.x);
        assert_eq!(2, coord.y);
    }

    #[test]
    fn is_adjacent() {
        is_adjacent_coords(0, 0, 1, 0, true);
        is_adjacent_coords(0, 0, 1, 1, true);
        is_adjacent_coords(-1, 0, 0, 0, true);

        is_adjacent_coords(-1, 0, 0, 5, false);
    }

    fn is_adjacent_coords(x1: i8, y1: i8, x2: i8, y2: i8, expected: bool) {
        let result = Coordinate::is_adjacent(Coordinate::new(x1, y1),
                                             Coordinate::new(x2, y2));
        assert!(result == expected)
    }
}
