use crate::model::common::*;
use bevy::math::Rect;

pub struct Unibody {
    pub rect: Rect<i32>,
    pub units: Vec<Point>,
    pub lookup: Lookup,
}

#[allow(dead_code)]
impl Unibody {
    pub fn from<'a, I, U, V>(it: I) -> Self
    where
        I: Iterator<Item = &'a U>,
        U: 'a + Location<V>,
        V: Into<i32>,
    {
        // [0] collect points into units
        let mut units: Vec<Point> = it.map(Point::from).collect();

        // [1] create rect
        let rect = Rect {
            left: units.iter().map(|u| u.x).min().unwrap_or_default(),
            right: units.iter().map(|u| u.x).max().unwrap_or_default(),
            top: units.iter().map(|u| u.y).min().unwrap_or_default(),
            bottom: units.iter().map(|u| u.y).max().unwrap_or_default(),
        };

        // [2] update unit
        for unit in units.iter_mut() {
            unit.x -= rect.left;
            unit.y -= rect.top;
        }

        // [3] create lookup table and update CubePattern
        let lookup = Lookup::from(units.iter());

        // [4] finish
        Self {
            rect,
            units,
            lookup,
        }
    }

    pub fn calculate_patterns(&self) -> impl Iterator<Item = (&Point, CubePattern)> {
        self.units
            .iter()
            .map(|unit| {
                let mut pattern = CubePattern::new();
                for direction in CubePattern::AROUND {
                    if self.lookup.get(&unit.near(direction)).is_some() {
                        pattern.set(direction);
                    }
                }
                (unit, pattern)
            })
            .into_iter()
    }

    // TODO: implement merge methods.
    // pub fn merge(&mut self, that: Self) {}
}
