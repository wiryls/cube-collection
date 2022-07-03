use std::{
    collections::{HashSet, VecDeque},
    rc::Rc,
};

use super::{
    Action, Background, BasicCube, CollectedCube, Collection, Conflict, DisjointSet, HeadID,
    Movement, Restriction,
};

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
        let get = self.active.getter(None);
        let mut groups = DisjointSet::new(get.number_of_cubes());
        for cube in get.cubes().filter(BasicCube::unstable) {
            for other in cube.neighbors() {
                if cube.absorbable_actively(&other) {
                    groups.join(&cube, &other);
                }
            }
        }

        // create next states
        Self {
            active: self.active.transform(groups, None),
            frozen: self.frozen.clone(),
        }
    }

    pub fn next(&self, input: Option<Movement>) -> Self {
        fn suppose<T: BasicCube>(
            cube: &T,
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

        let get = self.active.getter(input);
        let len = get.number_of_cubes();
        let moving = CollectedCube::into_movable;
        let movable = get.cubes().filter_map(moving);

        let mut groups = DisjointSet::new(len);
        let mut action = Box::<[_]>::from(vec![Restriction::Free; len]);
        let mut solved = HashSet::<HeadID>::with_capacity(len);

        // find blocked and build dependencies.
        // TODO: optimize the successors
        let mut successors = vec![HashSet::with_capacity(len); len].into_boxed_slice();
        for cube in movable.clone() {
            let mut blocked = cube.outlines_in_front().any(|o| self.frozen.blocked(o));

            if !blocked {
                // theoretically, neighbors are all movable.
                for neighbor in cube.neighbors_in_front().filter_map(moving) {
                    if neighbor.movement() == cube.movement() {
                        if !blocked {
                            successors[neighbor.index()].insert(cube.id());
                        }
                    } else if neighbor.absorbable(&cube) || cube.absorbable(&neighbor) {
                        if groups.join(&cube, &neighbor) {
                            blocked = true;
                        }
                    }
                }
            }

            if blocked {
                suppose(&cube, Restriction::Stop, &mut solved, &mut action);
            }
        }

        // build conflict table and find knocked.
        let conflict = {
            let mut it = Conflict::with_capacity(get.number_of_units());
            movable
                .filter(|cube| action[cube.index()] == Restriction::Free)
                .for_each(|cube| it.put(cube.id(), cube.movement(), cube.outlines_in_front()));
            it
        };
        for overleap in conflict.overlaps() {
            let cubes = overleap.map(|x| get.cube(x.0));
            if let Some(cube) = cubes
                .clone()
                .filter_map(moving)
                .find(|cube| cubes.clone().all(|other| cube.absorbable_actively(&other)))
            {
                let movement = cube.movement();
                for other in cubes.filter_map(moving) {
                    if movement.is_orthogonal(other.movement()) {
                        suppose(&other, Restriction::Stop, &mut solved, &mut action);
                    } else if movement.is_opposite(other.movement()) {
                        suppose(&other, Restriction::Lock, &mut solved, &mut action);
                    }
                }
            } else {
                for cube in cubes {
                    suppose(&cube, Restriction::Stop, &mut solved, &mut action);
                }
            }
        }

        // solve dependencies
        let mut queue = VecDeque::with_capacity(get.number_of_cubes());
        for cube in solved {
            queue.push_back(get.cube(cube));
            while let Some(cube) = queue.pop_back() {
                let cubes = successors[cube.index()].iter();
                if cube.unstable() {
                    cubes
                        .clone()
                        .map(|other| get.cube(other.clone()))
                        .filter(|other| cube.absorbable(other) || other.absorbable(&cube))
                        .for_each(|other| {
                            groups.join(&cube, &other);
                        });
                }

                let restriction = action[cube.index()];
                for other in cubes {
                    let this = &mut action[usize::from(other)];
                    if *this < restriction {
                        *this = restriction;
                        queue.push_back(get.cube(other.clone()));
                    }
                }
            }
        }

        // output
        let action = get
            .cubes()
            .map(|u| (action[u.index()], u))
            .map(|(r, u)| u.movable().map(move |m| Action::new(m, r)))
            .collect::<Vec<_>>();

        Self {
            active: self.active.transform(groups, Some(&action)),
            frozen: self.frozen.clone(),
        }
    }
}
