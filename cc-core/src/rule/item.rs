use crate::cube::{Constraint, Kind, Movement, Neighborhood, Point};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Item {
    pub id: usize,
    pub kind: Kind,
    pub position: Point,
    pub movement: Option<Movement>,
    pub constraint: Constraint,
    pub neighborhood: Neighborhood,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Diff {
    pub id: usize,
    pub kind: Option<Kind>,
    pub position: Option<Point>,
    pub movement: Option<Option<Movement>>,
    pub constraint: Option<Constraint>,
    pub neighborhood: Option<Neighborhood>,
}
