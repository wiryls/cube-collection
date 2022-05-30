use super::direction::Direction;

pub trait Location<T> {
    fn x_(&self) -> T;
    fn y_(&self) -> T;
}

#[derive(Clone, Copy, Default)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub fn new<T: Into<i32>>(x: T, y: T) -> Self {
        Self {
            x: x.into(),
            y: y.into(),
        }
    }

    pub const fn next(&self, dir: Direction) -> Self {
        match dir {
            Direction::LEFT => Self {
                x: self.x - 1,
                y: self.y,
            },
            Direction::LEFT_TOP => Self {
                x: self.x - 1,
                y: self.y - 1,
            },
            Direction::TOP => Self {
                x: self.x,
                y: self.y - 1,
            },
            Direction::RIGHT_TOP => Self {
                x: self.x + 1,
                y: self.y - 1,
            },
            Direction::RIGHT => Self {
                x: self.x + 1,
                y: self.y,
            },
            Direction::RIGHT_BOTTOM => Self {
                x: self.x + 1,
                y: self.y + 1,
            },
            Direction::BOTTOM => Self {
                x: self.x,
                y: self.y + 1,
            },
            Direction::LEFT_BOTTOM => Self {
                x: self.x - 1,
                y: self.y + 1,
            },
            _ => Self {
                x: self.x,
                y: self.y,
            },
        }
    }
}

impl Location<i32> for Point {
    fn x_(&self) -> i32 {
        self.x
    }

    fn y_(&self) -> i32 {
        self.y
    }
}
