use super::lookup::{Location, Lookup};
use super::near::{Direction, Near};
use bevy::math::Rect;

#[derive(Default)]
pub struct United {
    pub rect: Rect<i32>,
    pub units: Vec<Unit>,
    pub lookup: Lookup,
}

#[allow(dead_code)]
impl United {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn from<'a, I, U, V>(it: I) -> Self
    where
        I: Iterator<Item = &'a U>,
        U: 'a + Location<V>,
        V: Into<i32>,
    {
        // [0] collect points into units
        let mut units: Vec<Unit> = it
            .map(|point| Unit {
                n: Near::new(),
                o: Point {
                    x: point.x_().into(),
                    y: point.y_().into(),
                },
            })
            .collect();

        // [1] create rect
        let mut rect = units
            .first()
            .map(|u| Rect {
                left: u.o.x,
                right: u.o.x,
                top: u.o.y,
                bottom: u.o.y,
            })
            .unwrap_or_default();
        for v in units.iter().skip(1) {
            rect.left = rect.left.min(v.o.x);
            rect.right = rect.right.max(v.o.x);
            rect.top = rect.top.min(v.o.y);
            rect.bottom = rect.bottom.max(v.o.y);
        }

        // [2] update unit.o
        for unit in units.iter_mut() {
            unit.o.x -= rect.left;
            unit.o.y -= rect.top;
        }

        // [3] create lookup table and update unit.n
        let lookup = Lookup::from(units.iter());
        for v in units.iter_mut() {
            for d in Near::AROUND {
                if lookup.get(&v.o.step(d)).is_some() {
                    v.n.set(d);
                }
            }
        }

        // finish
        Self {
            rect,
            units,
            lookup,
        }
    }

    pub fn merge(&mut self, that: Self) {}
}

#[derive(Default)]
pub struct Unit {
    pub n: Near,
    pub o: Point,
}

impl Location<i32> for Unit {
    fn x_(&self) -> i32 {
        self.o.x
    }

    fn y_(&self) -> i32 {
        self.o.y
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

    pub const fn step(&self, dir: Direction) -> Self {
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
