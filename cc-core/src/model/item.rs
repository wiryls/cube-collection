use super::{Action, Kind};
use crate::common::{Neighborhood, Point};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Item {
    pub id: usize,
    pub kind: Kind,
    pub action: Option<Action>,
    pub position: Point,
    pub neighborhood: Neighborhood,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Diff {
    pub id: usize,
    pub kind: Option<Kind>,
    pub action: Option<Option<Action>>,
    pub position: Option<Point>,
    pub neighborhood: Option<Neighborhood>,
}
