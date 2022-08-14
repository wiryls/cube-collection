use super::{lookup::Collision, neighborhood::Neighborhood, point::Point};

pub trait CollisionExtension {
    fn neighborhood(&self, point: Point) -> Neighborhood;
}

impl<T: Collision> CollisionExtension for T {
    fn neighborhood(&self, point: Point) -> Neighborhood {
        Neighborhood::from(
            Neighborhood::AROUNDS
                .into_iter()
                .filter(|o| self.hit(point + o.into())),
        )
    }
}
