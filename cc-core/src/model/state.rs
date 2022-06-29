use std::rc::Rc;

use super::{Background, CollectedCube, Collection, DisjointSet, Movement, Restriction};

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
        let mut merged = DisjointSet::new(self.active.len());
        for cube in self.active.cubes().filter(CollectedCube::unstable) {
            cube.neighbors(Movement::Idle)
                .filter(|that| cube.absorbable_actively(that))
                .for_each(|that| merged.join(&cube, &that));
        }

        // create next states
        Self {
            active: self.active.transform(merged, None),
            frozen: self.frozen.clone(),
        }
    }

    pub fn next(&self, choice: Option<Movement>) -> Self {
        let limit = self.active.len();

        let mut merged = DisjointSet::new(limit);
        let mut solved = Vec::with_capacity(limit);
        let mut action = vec![Restriction::Free; limit];

        let moving = self.active.cubes().filter(CollectedCube::moving);
        for cube in moving.clone() {
            let movement = cube.movement();

            let blocked = cube.outlines(movement).any(|o| self.frozen.blocked(o));
            if blocked {
                // wall
                action[cube.index()] = Restriction::Block;
                solved.push(cube.index());
            } else {
                // neighbors
                cube.neighbors(movement)
                    .filter(|that| that.movement() != movement)
                    .filter(|that| that.absorbable(&cube) || cube.absorbable(that))
                    .for_each(|that| merged.join(&cube, &that));
            }

            // add to seeds
            // add dependencies
        }

        todo!()
    }
}
