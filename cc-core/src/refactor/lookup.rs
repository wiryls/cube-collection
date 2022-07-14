use std::{borrow::Borrow, collections::HashSet};

use super::{neighborhood::Neighborhood, point::Point};

/////////////////////////////////////////////////////////////////////////////
// Collision

pub trait Collision {
    fn hit(&self, point: Point) -> bool;
    fn put(&mut self, point: Point);
}

pub struct HashSetCollision(HashSet<Point>);

impl HashSetCollision {
    pub fn new<T: Borrow<Point>, I: Iterator<Item = T>>(it: I) -> Self {
        Self(it.map(|x| x.borrow().clone()).collect())
    }
}

impl Collision for HashSetCollision {
    fn hit(&self, point: Point) -> bool {
        self.0.contains(&point)
    }

    fn put(&mut self, point: Point) {
        self.0.insert(point);
    }
}

#[derive(Debug)]
pub struct BitmapCollision {
    width: i32,
    height: i32,
    bits: Box<[u64]>,
}

impl BitmapCollision {
    const UNIT: usize = 64;

    pub fn new(width: usize, height: usize) -> Self {
        let size = (width.max(1) * height.max(1) + Self::UNIT - 1) / Self::UNIT;
        Self {
            width: width as i32,
            height: height as i32,
            bits: vec![0; size].into(),
        }
    }

    fn collapse(&self, point: Point) -> Option<(usize, usize)> {
        if 0 <= point.x && point.x < self.width && 0 <= point.y && point.y < self.height {
            let index = (point.x + point.y * self.width) as usize;
            Some((index / Self::UNIT, index % Self::UNIT))
        } else {
            None
        }
    }
}

impl Collision for BitmapCollision {
    fn hit(&self, point: Point) -> bool {
        match self.collapse(point) {
            Some((index, delta)) => self.bits[index] & (1 << delta) != 0,
            None => false,
        }
    }

    fn put(&mut self, point: Point) {
        if let Some((index, delta)) = self.collapse(point) {
            self.bits[index] |= 1 << delta;
        }
    }
}

pub trait CollisionExtension {
    fn neighborhood(&self, point: Point) -> Neighborhood;
}

impl<T: Collision> CollisionExtension for T {
    fn neighborhood(&self, point: Point) -> Neighborhood {
        Neighborhood::from(
            Neighborhood::AROUND
                .into_iter()
                .filter(|o| self.hit(point + o.into())),
        )
    }
}

/////////////////////////////////////////////////////////////////////////////
// Tests

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn collisions() {
        fn case<C: Collision>(mut it: C, tag: &'static str) {
            let put = [(1, 1), (1, 2), (4, 2)].map(Point::from);
            let not = [(-1, -1), (0, 0), (1, 0), (0, 1), (1, 4)].map(Point::from);

            for o in put {
                it.put(o);
            }
            for o in put {
                assert!(it.hit(o), "{} {:?}", tag, o);
            }
            for o in not {
                assert!(!it.hit(o), "{} {:?}", tag, o);
            }
        }

        case(BitmapCollision::new(5, 3), "5x3 bitmap");
        case(BitmapCollision::new(10, 10), "10x10 bitmap");
        case(HashSetCollision::new::<Point, _>([].into_iter()), "hashset");
    }
}
