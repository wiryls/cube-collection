use super::Collision;
use crate::common::{Adjacent, Neighborhood, Point};

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
                points.map(move |point| {
                    (
                        point,
                        Neighborhood::from(
                            Neighborhood::AROUND
                                .into_iter()
                                .filter(|&a| collision.hit(point.near(a))),
                        ),
                    )
                })
            })
            .collect::<Box<_>>();
        let block = Collision::new(units.iter().map(|u| u.0));

        Self { units, block }
    }

    pub fn blocked(&self, point: Point) -> bool {
        self.block.hit(point)
    }
}
