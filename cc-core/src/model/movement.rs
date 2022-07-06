use crate::common::Point;

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub enum Movement {
    Left,
    Down,
    Up,
    Right,
}

impl Movement {
    const LEFT: Point = Point::new(-1, 0);
    const DOWN: Point = Point::new(0, 1);
    const UP: Point = Point::new(0, -1);
    const RIGHT: Point = Point::new(1, 0);

    pub fn opposite(&self) -> Self {
        use Movement::*;
        match self {
            Left => Right,
            Down => Up,
            Up => Down,
            Right => Left,
        }
    }

    pub fn is_opposite(&self, other: Self) -> bool {
        self.opposite() == other
    }

    pub fn is_orthogonal(&self, other: Self) -> bool {
        use Movement::*;
        match self {
            Left | Right => matches!(other, Up | Down),
            Down | Up => matches!(other, Left | Right),
        }
    }
}

impl Into<Point> for Movement {
    fn into(self) -> Point {
        match self {
            Movement::Left => Movement::LEFT,
            Movement::Down => Movement::DOWN,
            Movement::Up => Movement::UP,
            Movement::Right => Movement::RIGHT,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Restriction {
    Free,
    Lock,
    Stop,
}

impl Default for Restriction {
    fn default() -> Self {
        Self::Free
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Action {
    pub movement: Movement,
    pub restriction: Restriction,
}

impl Action {
    pub fn new(movement: Movement, restriction: Restriction) -> Self {
        Self {
            movement,
            restriction,
        }
    }
}
