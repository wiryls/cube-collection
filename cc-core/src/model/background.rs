use super::{Collision, Item, Kind};
use crate::common::{Neighborhood, Point};

pub struct Background {
    units: Box<[(Point, Neighborhood)]>,
    block: Collision,
}

impl Background {
    pub fn new<I, T>(it: I) -> Self
    where
        I: Iterator<Item = T>,
        T: Iterator<Item = Point> + Clone,
    {
        let units = it
            .flat_map(|points| {
                let collision = Collision::new(points.clone());
                points.map(move |point| (point, collision.neighborhood(point)))
            })
            .collect::<Box<_>>();
        let block = Collision::new(units.iter().map(|u| u.0));

        Self { units, block }
    }

    pub fn blocked(&self, point: Point) -> bool {
        self.block.hit(point)
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
