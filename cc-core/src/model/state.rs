use std::rc::Rc;

use super::{Collection, Movement};
use crate::common::Collision;

pub struct State {
    active: Collection,
    stable: Rc<Collection>,
    closed: Rc<Collision>,
}

impl State {
    pub fn current() {}
    pub fn diff(&self, that: &Self) /* -> Diff */ {}
    pub fn link(&self) /* -> Self */ {}
    pub fn next(&self, movement: Movement) /* -> Self */ {}
}
