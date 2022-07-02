use std::rc::Rc;

use crate::Conflict;

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
        let mut merged = DisjointSet::new(self.active.number_of_cubes());
        for cube in self.active.cubes().filter(CollectedCube::unstable) {
            cube.neighbors(Movement::Idle)
                .filter(|that| cube.absorbable_actively(that))
                .for_each(|that| {
                    merged.join(&cube, &that);
                });
        }

        // create next states
        Self {
            active: self.active.transform(merged, None),
            frozen: self.frozen.clone(),
        }
    }

    pub fn next(&self, choice: Option<Movement>) -> Self {
        let limit = self.active.number_of_cubes();
        let moving = self.active.cubes().filter(CollectedCube::moving);

        let mut merged = DisjointSet::new(limit);
        let mut action = Box::<[_]>::from(vec![Restriction::Free; limit]);
        let mut solved = Vec::with_capacity(limit);
        let mut behind = Box::<[_]>::from(vec![Vec::<CollectedCube>::with_capacity(limit); limit]);

        // find blocked.
        for cube in moving.clone() {
            let movement = cube.movement();
            let mut blocked = cube.outlines(movement).any(|o| self.frozen.blocked(o));

            if !blocked {
                for neighbor in cube.neighbors(movement) {
                    if neighbor.movement() == movement {
                        if !blocked {
                            behind[neighbor.index()].push(cube.clone());
                        }
                    } else if neighbor.absorbable(&cube) || cube.absorbable(&neighbor) {
                        if merged.join(&cube, &neighbor) {
                            blocked = true;
                        }
                    }
                }
            }

            if blocked {
                action[cube.index()] = Restriction::Block;
                solved.push(cube.index());
            }
        }

        // build conflict table and find locked.
        let conflict = {
            let mut it = Conflict::with_capacity(self.active.number_of_units());
            moving
                .clone()
                .filter(|cube| action[cube.index()] != Restriction::Block)
                .for_each(|cube| {
                    it.put(cube.id(), cube.movement(), cube.outlines(cube.movement()))
                });
            it
        };

        todo!()
    }
}
