use super::point::Point;

#[allow(dead_code)]
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub enum Movement {
    Left,
    Down,
    Up,
    Right,
}

#[allow(dead_code)]
impl Movement {
    pub const ALL: [Movement; 4] = [
        Movement::Left,
        Movement::Down,
        Movement::Up,
        Movement::Right,
    ];

    pub fn opposite(&self) -> Self {
        use Movement::*;
        match self {
            Left => Right,
            Down => Up,
            Up => Down,
            Right => Left,
        }
    }

    pub fn opposite_to(&self, other: Self) -> bool {
        self.opposite() == other
    }

    pub fn orthogonal_to(&self, other: Self) -> bool {
        use Movement::*;
        match self {
            Left | Right => matches!(other, Up | Down),
            Down | Up => matches!(other, Left | Right),
        }
    }
}

impl Into<Point> for Movement {
    fn into(self) -> Point {
        const LEFT: Point = Point::new(-1, 0);
        const DOWN: Point = Point::new(0, 1);
        const UP: Point = Point::new(0, -1);
        const RIGHT: Point = Point::new(1, 0);
        use Movement::*;
        match self {
            Left => LEFT,
            Down => DOWN,
            Up => UP,
            Right => RIGHT,
        }
    }
}

#[allow(dead_code)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Constraint {
    Free, // free to move
    Pong, // hit other cubes
    Lock, // blocked as competing on the same point
    Stop, // obstacles on the path
}

impl Default for Constraint {
    fn default() -> Self {
        Self::Free
    }
}
