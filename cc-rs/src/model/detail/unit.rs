use crate::model::common::{Location, Lookup, Point, Vicinity};
use bevy::math::Rect;

#[derive(Default)]
pub struct Unit {
    pub v: Vicinity,
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

impl Unit {
    fn from<T, U>(o: &T) -> Self
    where
        T: Location<U>,
        U: Into<i32>,
    {
        Self {
            v: Vicinity::new(),
            o: Point {
                x: o.x_().into(),
                y: o.y_().into(),
            },
        }
    }
}

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
        let mut units: Vec<Unit> = it.map(Unit::from).collect();

        // [1] create rect
        let rect = Rect {
            left: units.iter().map(|u| u.o.x).min().unwrap_or_default(),
            right: units.iter().map(|u| u.o.x).max().unwrap_or_default(),
            top: units.iter().map(|u| u.o.y).min().unwrap_or_default(),
            bottom: units.iter().map(|u| u.o.y).max().unwrap_or_default(),
        };

        // [2] update unit.o
        for unit in units.iter_mut() {
            unit.o.x -= rect.left;
            unit.o.y -= rect.top;
        }

        // [3] create lookup table and update vicinity
        let lookup = Lookup::from(units.iter());
        for u in units.iter_mut() {
            for v in Vicinity::AROUND {
                if lookup.get(&u.o.near(v)).is_some() {
                    u.v.set(v);
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

    // TODO: implement merge methods.
    // pub fn merge(&mut self, that: Self) {}
}
