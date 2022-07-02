use crate::common::Point;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
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

    pub fn is_opposite(&self, other: Self) -> bool {
        use Movement::*;
        match self {
            Left => other == Right,
            Down => other == Up,
            Up => other == Down,
            Right => other == Left,
            Idle => false,
        }
    }

    pub fn is_orthogonal(&self, other: Self) -> bool {
        use Movement::*;
        match self {
            Left | Right => other == Up || other == Down,
            Down | Up => other == Left || other == Right,
            Idle => false,
        }
    }
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

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
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
