use super::lookup::{Location, Lookup};
use super::near::{Direction, Near};

#[derive(Default)]
pub struct Group {
    pub units: Vec<Unit>,
    pub lookup: Lookup,
}

impl Group {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn from<'a, I, U, V>(it: I) -> Self
    where
        I: Iterator<Item = &'a U>,
        U: 'a + Location<V>,
        V: Into<i32>,
    {
        let mut units: Vec<Unit> = it
            .map(|o| Unit {
                o: Point {
                    x: o.x_().into(),
                    y: o.y_().into(),
                },
                n: Near::new(),
            })
            .collect();

        let lookup = Lookup::from(units.iter());
        for v in units.iter_mut() {
            for d in Near::AROUND {
                if lookup.get(&v.o.step(d)).is_some() {
                    v.n.set(d);
                }
            }
        }

        Self { units, lookup }
    }

    pub fn merge(&mut self, that: Self) {}
}

#[derive(Default)]
pub struct Unit {
    pub o: Point,
    pub n: Near,
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
                y: self.y + 1,
            },
            Direction::TOP => Self {
                x: self.x,
                y: self.y + 1,
            },
            Direction::RIGHT_TOP => Self {
                x: self.x + 1,
                y: self.y + 1,
            },
            Direction::RIGHT => Self {
                x: self.x + 1,
                y: self.y,
            },
            Direction::RIGHT_BOTTOM => Self {
                x: self.x + 1,
                y: self.y - 1,
            },
            Direction::BOTTOM => Self {
                x: self.x,
                y: self.y - 1,
            },
            Direction::LEFT_BOTTOM => Self {
                x: self.x - 1,
                y: self.y - 1,
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
