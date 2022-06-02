/// The type of cubes.
#[derive(Clone, Copy, PartialEq)]
pub enum Type {
    White,
    Red,
    Blue,
    Green,
}

#[derive(Clone, Copy, PartialEq)]
pub enum Movement {
    Idle,
    Left,
    Down,
    Up,
    Right,
}
