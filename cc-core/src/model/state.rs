use std::{
    collections::{HashSet, VecDeque},
    rc::Rc,
};

use super::{Background, CollectedCube, Collection, DisjointSet, HeadID, Movement, Restriction};
use crate::Conflict;

pub struct State {
    active: Collection,
    frozen: Rc<Background>,
}

impl State {
    pub fn current() {}

    pub fn diff(&self, _that: &Self) /* -> Diff */
    {
        todo!()
    }

    pub fn link(&self) -> Self {
        // create set
        let mut merged = DisjointSet::new(self.active.number_of_cubes());
        for cube in self.active.cubes().filter(CollectedCube::unstable) {
            cube.neighbors()
                .filter(|other| cube.absorbable_actively(other))
                .for_each(|other| {
                    merged.join(&cube, &other);
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
        let mut solved = HashSet::<HeadID>::with_capacity(limit);

        fn determine(
            cube: &CollectedCube,
            restriction: Restriction,
            solved: &mut HashSet<HeadID>,
            action: &mut [Restriction],
        ) {
            let index = cube.index();
            if solved.insert(cube.id()) {
                action[index] = restriction;
            } else if action[index] < restriction {
                action[index] = restriction;
            }
        }

        // find blocked.
        let mut successors = vec![HashSet::with_capacity(limit); limit].into_boxed_slice();
        for cube in moving.clone() {
            let mut blocked = cube.outlines_ahead().any(|o| self.frozen.blocked(o));

            if !blocked {
                for neighbor in cube.neighbors_ahead() {
                    if neighbor.movement() == cube.movement() {
                        if !blocked {
                            successors[neighbor.index()].insert(cube.id());
                        }
                    } else if neighbor.absorbable(&cube) || cube.absorbable(&neighbor) {
                        if merged.join(&cube, &neighbor) {
                            blocked = true;
                        }
                    }
                }
            }

            if blocked {
                determine(&cube, Restriction::Block, &mut solved, &mut action);
            }
        }

        // build conflict table and find locked.
        let conflict = {
            let mut it = Conflict::with_capacity(self.active.number_of_units());
            moving
                .clone()
                .filter(|cube| action[cube.index()] != Restriction::Block)
                .for_each(|cube| it.put(cube.id(), cube.movement(), cube.outlines_ahead()));
            it
        };
        for overleap in conflict.overlaps() {
            let cubes = overleap.map(|x| self.active.cube(x.0));
            if let Some(cube) = cubes
                .clone()
                .find(|cube| cubes.clone().all(|other| cube.absorbable_actively(&other)))
            {
                let movement = cube.movement();
                for other in cubes {
                    if movement.is_orthogonal(other.movement()) {
                        determine(&other, Restriction::Block, &mut solved, &mut action);
                    } else if movement.is_opposite(other.movement()) {
                        determine(&other, Restriction::Knock, &mut solved, &mut action);
                    }
                }
            } else {
                for cube in cubes {
                    determine(&cube, Restriction::Block, &mut solved, &mut action);
                }
            }
        }

        // solve dependencies
        let mut queue = VecDeque::with_capacity(self.active.number_of_cubes());
        for cube in solved {
            queue.push_back(self.active.cube(cube));
            while let Some(cube) = queue.pop_back() {
                let cubes = successors[cube.index()].iter();
                if cube.unstable() {
                    cubes
                        .clone()
                        .map(|other| self.active.cube(other.clone()))
                        .filter(|other| cube.absorbable(other) || other.absorbable(&cube))
                        .for_each(|other| {
                            merged.join(&cube, &other);
                        });
                }

                let restriction = action[cube.index()];
                for other in cubes {
                    let other = &mut action[usize::from(other)];
                    if *other < restriction {
                        *other = restriction;
                    }
                }
            }
        }

        // output
        Self {
            active: self.active.transform(merged, Some(&action)),
            frozen: self.frozen.clone(),
        }
    }
}
