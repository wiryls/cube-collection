use std::rc::Rc;

use super::{Behavior, Borders, Movement, Type};
use crate::common::{Collision, Neighborhood, Point};

pub struct State {
    active: Collection,
    stable: Rc<Collection>,
    closed: Rc<Collision>,
}

impl State {
    pub fn current() {}
    pub fn next(movement: Movement) /* -> patch */ {}
}

pub type UnitID = usize;
pub type HeadID = usize;

struct Collection {
    heads: Vec<Head>,
    units: Box<[Unit]>,
}

struct Head {
    // necessary
    kind: Type,
    units: Vec<UnitID>,
    behavior: Option<Behavior>,
    // temporary
    edges: Option<Borders>,
}

struct Unit {
    head: HeadID,
    position: Point,
    neighborhood: Neighborhood,
}

struct Cache {}
