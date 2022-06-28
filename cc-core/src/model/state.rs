use std::rc::Rc;

use super::{
    Background, CollectedCube, Collection, DisjointSet, Movement, Restriction, Restrictions,
};

pub struct State {
    active: Collection,
    frozen: Rc<Background>,
}

impl State {
    pub fn current() {}

    pub fn diff(&self, that: &Self) /* -> Diff */
    {
        todo!()
    }

    pub fn link(&self) -> Self {
        // create set
        let mut merge = DisjointSet::new(self.active.len());
        for cube in self.active.cubes().filter(CollectedCube::unstable) {
            cube.neighbors(Movement::Idle)
                .filter(|that| cube.absorbable_actively(that))
                .for_each(|that| merge.join(cube.id(), that.id()));
        }

        // create next states
        Self {
            active: self.active.transform(merge, None),
            frozen: self.frozen.clone(),
        }
    }

    pub fn next(&self, choice: Option<Movement>) -> Self {
        let mut merge = DisjointSet::new(self.active.len());
        let mut action = Restrictions::new(&self.active);
        let moving = self.active.cubes().filter(CollectedCube::moving);

        for cube in moving.clone() {
            let movement = cube.movement();

            let blocked = cube.outlines(movement).any(|o| self.frozen.blocked(o));
            if blocked {
                // wall
                action.set(&cube, Restriction::Block);
            } else {
                // neighbors
                cube.neighbors(movement)
                    .filter(|that| that.movement() != movement)
                    .filter(|that| that.absorbable(&cube) || cube.absorbable(that))
                    .for_each(|that| merge.join(cube.id(), that.id()));
            }

            // add to seeds
            // add dependencies
        }

        todo!()
    }
}
