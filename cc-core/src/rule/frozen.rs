use super::{
    extension::CollisionExtension,
    lookup::{BitmapCollision, Collision, HashSetCollision},
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
        let cubes = {
            let build = |os: &'a [Point]| {
                let c = HashSetCollision::new(os.iter());
                os.iter().map(move |&o| (o, c.neighborhood(o)))
            };
            it.flat_map(build).collect::<Box<_>>()
        };

        let collision = {
            let mut it = BitmapCollision::new(width, height);
            cubes.iter().for_each(|x| it.put(x.0));
            it
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
}
