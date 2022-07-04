use super::{Collision, Item, Kind};
use crate::common::{Neighborhood, Point};

pub struct Background {
    range: (i32, i32),
    units: Box<[(Point, Neighborhood)]>,
    block: Collision,
}

impl Background {
    pub fn new<I, T>(range: (usize, usize), it: I) -> Self
    where
        I: Iterator<Item = T>,
        T: Iterator<Item = Point> + Clone,
    {
        let range = (range.0 as i32, range.1 as i32);
        let units = it
            .flat_map(|points| {
                let collision = Collision::new(points.clone());
                points.map(move |point| {
                    (
                        point,
                        Neighborhood::from(
                            Neighborhood::AROUND
                                .into_iter()
                                .filter(|&a| collision.hit(point + a.into())),
                        ),
                    )
                })
            })
            .collect::<Box<_>>();
        let block = Collision::new(units.iter().map(|u| u.0));

        Self {
            range,
            units,
            block,
        }
    }

    pub fn blocked(&self, point: Point) -> bool {
        !(0 <= point.x
            && point.x < self.range.0
            && 0 <= point.y
            && point.y < self.range.1
            && !self.block.hit(point))
    }

    pub fn iter(&self, offset: usize) -> BackgroundIter {
        BackgroundIter {
            offset,
            iterator: self.units.iter().enumerate(),
        }
    }
}

pub struct BackgroundIter<'a> {
    offset: usize,
    iterator: std::iter::Enumerate<std::slice::Iter<'a, (Point, Neighborhood)>>,
}

impl Iterator for BackgroundIter<'_> {
    type Item = Item;

    fn next(&mut self) -> Option<Self::Item> {
        self.iterator
            .next()
            .map(|(index, (point, neighborhood))| Item {
                id: (index + self.offset).into(),
                kind: Kind::White,
                action: None,
                position: point.clone(),
                neighborhood: neighborhood.clone(),
            })
    }
}
