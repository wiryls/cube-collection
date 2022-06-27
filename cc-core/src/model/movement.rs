use crate::common::Point;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Movement {
    Idle,
    Left,
    Down,
    Up,
    Right,
}

impl Movement {
    const IDLE: Point = Point::new(0, 0);
    const LEFT: Point = Point::new(-1, 0);
    const DOWN: Point = Point::new(0, 1);
    const UP: Point = Point::new(0, -1);
    const RIGHT: Point = Point::new(1, 0);
}

impl Into<Point> for Movement {
    fn into(self) -> Point {
        match self {
            Movement::Idle => Movement::IDLE,
            Movement::Left => Movement::LEFT,
            Movement::Down => Movement::DOWN,
            Movement::Up => Movement::UP,
            Movement::Right => Movement::RIGHT,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Restriction {
    Free,
    Knock,
    Block,
}

impl Default for Restriction {
    fn default() -> Self {
        Self::Free
    }
}
