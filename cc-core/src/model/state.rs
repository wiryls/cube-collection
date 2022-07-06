use std::{
    collections::{HashSet, VecDeque},
    rc::Rc,
};

use super::{
    Action, Background, BasicCube, CollectedCube, Collection, Conflict, Diff, DisjointSet, HeadID,
    Item, Kind, Motion, Movement, Restriction, Successors,
};
use crate::{common::Point, Seed};

pub struct State {
    region: (i32, i32),
    active: Collection,
    frozen: Rc<Background>,
    target: Rc<Box<[Point]>>,
}

impl State {
    pub fn new(seed: &Seed) -> Self {
        let region = (seed.size.width, seed.size.height);
        let active = Collection::new(
            seed.cubes
                .iter()
                .filter(|cube| cube.kind != Kind::White || cube.command.is_none())
                .map(|cube| {
                    (
                        cube.kind,
                        cube.body.as_slice(),
                        match &cube.command {
                            None => Motion::new(),
                            Some(command) => Motion::from_sequence(
                                command.is_loop,
                                command.movements.iter().cloned(),
                            ),
                        },
                    )
                }),
        );
        let frozen = Background::new(
            seed.cubes
                .iter()
                .filter(|cube| cube.kind == Kind::White && cube.command.is_none())
                .map(|cube| cube.body.iter().cloned()),
        )
        .into();
        let target = seed.destnations.clone().into_boxed_slice().into();

        Self {
            region,
            active,
            frozen,
            target,
        }
    }

    pub fn current(&self) -> impl Iterator<Item = Item> + '_ {
        let offset = self.active.number_of_units();
        self.active.iter().chain(self.frozen.iter(offset))
    }

    pub fn differ<'a>(&'a self, that: &'a Self) -> impl Iterator<Item = Diff> + 'a {
        let comparable = self.region == that.region
            && self.active.number_of_units() == that.active.number_of_units()
            && std::ptr::eq(self.frozen.as_ref(), that.frozen.as_ref());

        let maximum = if comparable {
            self.active.number_of_units()
        } else {
            0
        };

        self.active
            .iter()
            .zip(that.active.iter())
            .filter(|(l, r)| l.id == r.id)
            .map(|(l, r)| Diff {
                id: r.id,
                kind: (l.kind != r.kind).then(|| r.kind),
                action: (l.action != r.action).then(|| r.action),
                position: (l.position != r.position).then(|| r.position),
                neighborhood: (l.neighborhood != r.neighborhood).then(|| r.neighborhood),
            })
            .take(maximum)
    }

    pub fn link(&self) -> Self {
        // create set
        let collected = self.active.collected(None);
        let mut groups = DisjointSet::new(self.active.number_of_cubes());
        for cube in collected.cubes().filter(BasicCube::unstable) {
            for other in cube.neighbors() {
                if cube.absorbable_actively(&other) {
                    groups.join(&cube, &other);
                }
            }
        }

        // create next states
        Self {
            region: self.region.clone(),
            active: self.active.transform(groups, None),
            frozen: self.frozen.clone(),
            target: self.target.clone(),
        }
    }

    pub fn next(&self, input: Option<Movement>) -> Self {
        fn suppose<T: BasicCube>(
            cube: &T,
            restriction: Restriction,
            solved: &mut HashSet<HeadID>,
            action: &mut [Restriction],
        ) {
            let action = &mut action[cube.index()];
            if solved.insert(cube.id()) || *action < restriction {
                *action = restriction;
            }
        }

        let number_of_cubes = self.active.number_of_cubes();
        let number_of_units = self.active.number_of_units();
        let collected = self.active.collected(input);
        let moving = CollectedCube::into_movable;

        let mut groups = DisjointSet::new(number_of_cubes);
        let mut action = Box::<[_]>::from(vec![Restriction::Free; number_of_cubes]);
        let mut solved = HashSet::<HeadID>::with_capacity(number_of_cubes);

        // find blocked and build dependencies.
        let mut successors = Successors::new(number_of_cubes);
        for cube in collected.cubes().filter_map(moving) {
            let mut blocked = cube
                .outlines_in_front()
                .any(|o| self.outside(o) || self.frozen.blocked(o));

            if !blocked {
                for neighbor in cube.neighbors_in_front() {
                    let id = neighbor.id();
                    let kind = neighbor.kind();
                    if let Some(neighbor) = neighbor.into_movable() {
                        if neighbor.movement() == cube.movement() {
                            if !blocked {
                                successors.insert(id, cube.id());
                            }
                            continue;
                        }
                    }
                    if kind.absorbable(cube.kind()) || cube.kind().absorbable(kind) {
                        groups.join(&cube, id);
                        blocked = true;
                    }
                }
            }

            if blocked {
                suppose(&cube, Restriction::Stop, &mut solved, &mut action);
            }
        }

        // build conflict table and find knocked.
        let conflict = {
            let mut it = Conflict::with_capacity(number_of_units);
            collected
                .cubes()
                .filter(|cube| action[cube.index()] == Restriction::Free)
                .filter_map(moving)
                .for_each(|cube| it.put(cube.id(), cube.movement(), cube.outlines_in_front()));
            it
        };
        for overleap in conflict.overlaps() {
            let cubes = overleap.map(|x| collected.cube(x.0));
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
        let mut queue = VecDeque::with_capacity(number_of_cubes);
        for cube in solved {
            queue.push_back(collected.cube(cube));
            while let Some(cube) = queue.pop_back() {
                let cubes = successors.walk(cube.id());
                if cube.unstable() {
                    cubes
                        .clone()
                        .map(|other| collected.cube(other.clone()))
                        .filter(|other| cube.absorbable(other) || other.absorbable(&cube))
                        .for_each(|other| groups.join(&cube, &other));
                }

                let restriction = action[cube.index()];
                for other in cubes {
                    let this = &mut action[usize::from(other)];
                    if *this < restriction {
                        *this = restriction;
                        queue.push_back(collected.cube(other.clone()));
                    }
                }
            }
        }

        // output
        let action = collected
            .cubes()
            .map(|u| (action[u.index()], u))
            .map(|(r, u)| u.movable().map(move |m| Action::new(m, r)))
            .collect::<Vec<_>>();

        Self {
            region: self.region.clone(),
            active: self.active.transform(groups, Some(&action)),
            frozen: self.frozen.clone(),
            target: self.target.clone(),
        }
    }

    fn outside(&self, o: Point) -> bool {
        !(0 <= o.x && o.x < self.region.0 && 0 <= o.y && o.y < self.region.1)
    }
}
