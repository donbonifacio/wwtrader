#[derive(Clone, Default, Debug, Copy)]
pub struct Direction {
    pub dx: f32,
    pub dy: f32,
}

pub const LEFT: Direction = Direction { dx: -1.0, dy: 0.0 };
pub const RIGHT: Direction = Direction { dx: 1.0, dy: 0.0 };
pub const UP: Direction = Direction { dx: 0.0, dy: -1.0 };
pub const DOWN: Direction = Direction { dx: 0.0, dy: 1.0 };

impl Direction {}
