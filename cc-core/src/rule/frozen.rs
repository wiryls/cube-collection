use super::{
    extension::CollisionExtension,
    lookup::{BitmapCollision, Collision},
};
use crate::cube::{Neighborhood, Point};

#[derive(Debug)]
pub struct Frozen {
    unchanged: Box<[(Point, Neighborhood)]>,
    collision: BitmapCollision,
}

impl Frozen {
    pub fn new<'a, I>(width: usize, height: usize, it: I) -> Self
    where
        I: Iterator<Item = &'a [Point]>,
    {
        let mut collision = BitmapCollision::new(width, height);
        let cubes = {
            let build = |os: &'a [Point]| {
                let mut c = BitmapCollision::new(width, height);
                os.iter().for_each(|&o| c.put(o));
                collision.or(&c);
                os.iter().map(move |&o| (o, c.neighborhood_or_border(o)))
            };
            it.flat_map(build).collect::<Box<_>>()
        };

        Self {
            unchanged: cubes,
            collision,
        }
    }

    pub fn blocked(&self, point: Point) -> bool {
        !self.collision.available(point)
    }

    pub fn iter<'a>(&'a self) -> std::slice::Iter<'a, (Point, Neighborhood)> {
        self.unchanged.iter()
    }

    pub fn len(&self) -> usize {
        self.unchanged.len()
    }

    pub fn width(&self) -> usize {
        self.collision.width()
    }

    pub fn height(&self) -> usize {
        self.collision.height()
    }
}
