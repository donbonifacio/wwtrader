#[derive(Clone, Default, Debug, Copy)]
pub struct Direction {
    pub dx: i8,
    pub dy: i8,
}

pub const LEFT: Direction = Direction { dx: -1, dy: 0 };
pub const RIGHT: Direction = Direction { dx: 1, dy: 0 };
pub const UP: Direction = Direction { dx: 0, dy: -1 };
pub const DOWN: Direction = Direction { dx: 0, dy: 1 };

impl Direction {}
