use super::pattern::Adjacence;

pub trait Location<T> {
    fn x(&self) -> T;
    fn y(&self) -> T;
}

impl<T: Copy> Location<T> for (T, T) {
    fn x(&self) -> T {
        self.0
    }
    fn y(&self) -> T {
        self.1
    }
}

#[derive(Clone, Copy, Default)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl<T: Location<i32>> From<&T> for Point {
    fn from(src: &T) -> Self {
        Self {
            x: src.x(),
            y: src.y(),
        }
    }
}

impl Location<i32> for Point {
    fn x(&self) -> i32 {
        self.x
    }
    fn y(&self) -> i32 {
        self.y
    }
}

#[allow(dead_code)]
impl Point {
    pub fn new<T: Into<i32>>(x: T, y: T) -> Self {
        Self {
            x: x.into(),
            y: y.into(),
        }
    }

    pub fn from<T, U>(o: &T) -> Self
    where
        T: Location<U>,
        U: Into<i32>,
    {
        Self {
            x: o.x().into(),
            y: o.y().into(),
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