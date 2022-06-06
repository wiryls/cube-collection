use std::{collections::HashSet, rc::Rc};

use super::{Behavior, Movement, Type};
use crate::common::{Neighborhood, Point};

pub struct State {
    active: Collection,
    stable: Rc<Collection>,
    cached: Option<()>,
}

impl State {
    pub fn current() {}
    pub fn next(movement: Movement) /* -> patch */ {}
}

type UnitID = usize;
type HeadID = usize;

struct Collection {
    heads: Vec<Head>,
    units: Box<[Unit]>,
}

struct Head {
    kind: Type,
    units: Vec<UnitID>,
    behavior: Option<Behavior>,
}

struct Unit {
    head: HeadID,
    position: Point,
    neighborhood: Neighborhood,
}
