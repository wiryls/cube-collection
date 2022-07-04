use super::{Action, Kind, UnitID};
use crate::common::{Neighborhood, Point};

#[derive(Clone)]
pub struct Item {
    pub id: UnitID,
    pub kind: Kind,
    pub action: Option<Action>,
    pub position: Point,
    pub neighborhood: Neighborhood,
}

pub struct Diff {
    pub id: UnitID,
    pub kind: Option<Kind>,
    pub action: Option<Option<Action>>,
    pub position: Option<Point>,
    pub neighborhood: Option<Neighborhood>,
}
