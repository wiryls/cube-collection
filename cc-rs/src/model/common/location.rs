use super::vicinity::Adjacence;

pub trait Location<T> {
    fn x_(&self) -> T;
    fn y_(&self) -> T;
}

impl Location<i32> for Point {
    fn x_(&self) -> i32 {
        self.x
    }

    fn y_(&self) -> i32 {
        self.y
    }
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

    pub const fn near(&self, dir: Adjacence) -> Self {
        match dir {
            Adjacence::LEFT => Self {
                x: self.x - 1,
                y: self.y,
            },
            Adjacence::LEFT_TOP => Self {
                x: self.x - 1,
                y: self.y - 1,
            },
            Adjacence::TOP => Self {
                x: self.x,
                y: self.y - 1,
            },
            Adjacence::RIGHT_TOP => Self {
                x: self.x + 1,
                y: self.y - 1,
            },
            Adjacence::RIGHT => Self {
                x: self.x + 1,
                y: self.y,
            },
            Adjacence::RIGHT_BOTTOM => Self {
                x: self.x + 1,
                y: self.y + 1,
            },
            Adjacence::BOTTOM => Self {
                x: self.x,
                y: self.y + 1,
            },
            Adjacence::LEFT_BOTTOM => Self {
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
