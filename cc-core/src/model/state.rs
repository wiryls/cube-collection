use std::rc::Rc;

use super::{Background, Collection, Collision, DisjointSet, FatCube, Movement};

pub struct State {
    active: Collection,
    frozen: Rc<Background>,
    closed: Rc<Collision>,
}

impl State {
    pub fn current() {}

    pub fn diff(&self, that: &Self) /* -> Diff */ {}

    pub fn link(&self) -> Self {
        // create set
        let mut set = DisjointSet::default();
        self.active
            .cubes()
            .filter(FatCube::unstable)
            .for_each(|cube| {
                cube.around(Movement::Idle)
                    .filter(|that| cube.absorbable_actively(that))
                    .for_each(|that| set.join(cube.id(), that.id()))
            });

        // create next states
        Self {
            active: self.active.next(set, None),
            frozen: self.frozen.clone(),
            closed: self.closed.clone(),
        }
    }

    pub fn next(&self, movement: Movement) /* -> Self */
    {
        // TODO:
    }
}

mod detail {}
