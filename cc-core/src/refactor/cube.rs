use std::{
    collections::{HashMap, HashSet, VecDeque},
    rc::Rc,
};

use super::{
    extension::CollisionExtension,
    item::Diff,
    item::Item,
    kind::Kind,
    lookup::{BitmapCollision, Collision, Digraph, DisjointSet, HashSetCollision},
    motion::{Agreement, Motion},
    movement::{Constraint, Movement},
    neighborhood::{Adjacence, Neighborhood},
    point::Point,
};

/////////////////////////////////////////////////////////////////////////////
// export

#[derive(Clone, Debug)]
pub struct View(Vec<Item>, usize);

impl View {
    pub fn iter(&self) -> core::slice::Iter<Item> {
        self.0.iter()
    }

    pub fn differ<'a>(&'a self, that: &'a Self) -> impl Iterator<Item = Diff> + 'a {
        let maximum = if self.0.len() == that.0.len() && self.1 == that.1 {
            self.0.len()
        } else {
            0
        };

        std::iter::zip(self.iter(), that.iter())
            .take(maximum)
            .filter(|(l, r)| {
                l.kind != r.kind
                    || l.position != r.position
                    || l.movement != r.movement
                    || l.constraint != r.constraint
                    || l.neighborhood != r.neighborhood
            })
            .map(|(l, r)| Diff {
                id: r.id,
                kind: (l.kind != r.kind).then(|| r.kind),
                position: (l.position != r.position).then(|| r.position),
                movement: (l.movement != r.movement).then(|| r.movement),
                constraint: (l.constraint != r.constraint).then(|| r.constraint),
                neighborhood: (l.neighborhood != r.neighborhood).then(|| r.neighborhood),
            })
    }

    pub fn contains(&self, position: Point) -> bool {
        self.0.iter().any(|unit| unit.position == position)
    }
}

#[derive(Clone, Debug)]
pub struct Collection {
    area: Rc<ImmutableArea>, // background and obstacles
    cube: Vec<Cube>,         // cubes (sets of units)
}

impl Collection {
    pub fn new<'a, I>(width: usize, height: usize, it: I) -> Self
    where
        I: Iterator<Item = (Kind, &'a [Point], Motion)> + 'a,
    {
        let mut count = 0;
        let mut cubes = Vec::new();
        let mut other = Vec::new();
        for (index, (kind, points, mut motion)) in it.enumerate() {
            let movement = match motion.next() {
                None if kind == Kind::White => {
                    other.push(points);
                    continue;
                }
                Some(movement) => movement,
                _ => None,
            };

            let collision = HashSetCollision::new(points.iter().cloned());
            let units = points
                .iter()
                .enumerate()
                .map(|(index, &point)| Unit {
                    index: index + count,
                    position: point,
                    neighborhood: collision.neighborhood(point),
                })
                .collect::<Vec<_>>();
            let contours = Rc::new(Contours::new(&units));

            let cube = Cube {
                index,
                kind,
                units,
                motion,
                contours,
                balanced: false,
                movement,
                constraint: Constraint::Free,
            };

            cubes.push(cube);
            count += points.len();
        }

        Self {
            area: Rc::new(ImmutableArea::new(width, height, other.into_iter())),
            cube: cubes,
        }
    }

    pub fn view(&self) -> View {
        let capacity = self.cube.len() + self.area.unchanged.len();
        let default = Item {
            id: 0,
            kind: Kind::White,
            position: Point::new(0, 0),
            movement: None,
            constraint: Constraint::Free,
            neighborhood: Neighborhood::new(),
        };
        let mut output = vec![default; capacity];
        for cube in self.cube.iter() {
            for unit in cube.units.iter() {
                output[unit.index] = Item {
                    id: unit.index,
                    kind: cube.kind,
                    position: unit.position,
                    movement: cube.movement,
                    constraint: cube.constraint,
                    neighborhood: unit.neighborhood,
                };
            }
        }
        let offset = self.cube.len();
        for (index, unit) in self.area.unchanged.iter().enumerate() {
            output[index + offset] = Item {
                id: index + offset,
                kind: Kind::White,
                position: unit.0,
                movement: None,
                constraint: Constraint::Free,
                neighborhood: unit.1,
            };
        }

        View(output, offset)
    }

    pub fn absorb(&mut self) {
        let number_of_cubes = self.cube.len();
        let unstable = self.cube.iter().filter(|u| u.alive() && u.unstable());

        let territory = Territory::new(unstable.clone());
        let mut connection = DisjointSet::new(number_of_cubes);

        // connect all adjacent cubes.
        let mut visit = vec![false; number_of_cubes];
        let mut queue = VecDeque::with_capacity(number_of_cubes);
        for cube in unstable {
            if !visit[cube.index] {
                visit[cube.index] = true;
                queue.push_back(cube);
            }

            while let Some(cube) = queue.pop_front() {
                for other in territory.neighbors(cube) {
                    if !visit[other.index] {
                        visit[other.index] = true;
                        queue.push_back(other);

                        if cube.mergeable(other) {
                            connection.join(cube, other);
                        }
                    }
                }
            }
        }

        // try to absorb each others.
        for group in connection.groups() {
            let mut arena = Arena::new(self);
            for &index in group.iter() {
                if !arena.input(index) {
                    break;
                }
            }
            use ArenaResult::*;
            match arena.output() {
                Have(kind) => self.merge(group, kind),
                Draw => group.into_iter().for_each(|i| self.cube[i].balanced = true),
                _ => {}
            };
        }

        self.bury();
    }

    pub fn input(&mut self, movement: Option<Movement>) {
        // update movements if not None.
        if let Some(movement) = movement {
            for cube in self.cube.iter_mut().filter(|cube| cube.kind == Kind::Green) {
                cube.movement = Some(movement);
            }
        }

        // shared tables.
        let number_of_cubes = self.cube.len();
        let mut connection = DisjointSet::new(number_of_cubes);
        let mut successors = Digraph::with_capacity(number_of_cubes);

        // find blocked and marks them with Constraint::Stop.
        let territory = Territory::new(self.cube.iter().filter(|cube| cube.alive()));
        let mut stopped = Vec::new();
        for cube in self.cube.iter().filter_map(Moving::new) {
            let mut blocked = cube.frontlines().any(|o| self.area.blocked(o));

            if !blocked {
                let neighbors = territory.neighbors_in_front(&cube).collect::<HashSet<_>>();
                blocked = neighbors
                    .iter()
                    .any(|&other| !cube.same_movement(other) && !cube.linkable(other));

                if !blocked {
                    for &other in neighbors.iter() {
                        if !cube.same_movement(other) && cube.linkable(other) {
                            blocked = true;
                            stopped.push(other.index);
                            connection.join(&cube, other);
                        }
                    }
                }

                if !blocked {
                    for &other in neighbors.iter() {
                        if cube.same_movement(other) {
                            successors.add(other, &cube);
                        }
                    }
                }
            }

            if blocked {
                stopped.push(cube.index);
            }
        }

        self.connect(stopped, &successors, Constraint::Stop, &mut connection)
            .into_iter()
            .for_each(|index| self.cube[index].constraint = Constraint::Stop);
        self.link(&mut connection);

        // find conflicts and marks them with Constraint::Lock.
        let mut conflict = Conflict::with_capacity(number_of_cubes);
        self.cube
            .iter()
            .filter_map(Moving::new)
            .filter(|cube| cube.constraint <= Constraint::Lock)
            .for_each(|cube| conflict.put(&cube, cube.movement, cube.frontlines()));

        let mut locked = HashSet::with_capacity(number_of_cubes);
        let mut impact = HashSet::with_capacity(number_of_cubes);
        let mut undetermined = HashSet::with_capacity(number_of_cubes);
        for race in conflict.overlaps() {
            let cube = &self.cube;
            let size = race.len();
            let half = size >> 1;
            for i in 0..race.len() {
                let it = match race[i] {
                    Some(index) => index,
                    ___________ => continue,
                };

                if cube[it].constraint > Constraint::Lock || locked.contains(&it) {
                    continue;
                }

                let prev = race[(i + size - 1) % size];
                let next = race[(i + /* **/ 1) % size];
                if Conflict::locked(cube, it, prev) || Conflict::locked(cube, it, next) {
                    locked.insert(it);
                    continue;
                }

                match race[(i + half) % size] {
                    None => (),
                    Some(that) if cube[it].absorbable(&cube[that]) => drop(undetermined.insert(it)),
                    Some(____) => drop(impact.insert(it)),
                };
            }
        }

        self.connect(locked, &successors, Constraint::Lock, &mut connection)
            .into_iter()
            .for_each(|index| self.cube[index].constraint = Constraint::Lock);
        self.link(&mut connection);

        // find impact position and marks them with Constraint::Slap.
        self.cube.iter_mut().for_each(|cube| cube.balanced = false);
        let territory = QuarterTerritory::new(self.cube.iter().filter(|u| u.alive()));

        let mut visit = vec![false; number_of_cubes];
        let mut queue = VecDeque::with_capacity(number_of_cubes);
        for index in undetermined.iter().cloned() {
            if !visit[index] {
                visit[index] = true;
                queue.push_back(index);
            }

            while let Some(index) = queue.pop_front() {
                let cube = &self.cube[index];
                for other in territory.neighbors(cube) {
                    if !visit[other.index] {
                        visit[other.index] = true;
                        queue.push_back(other.index);

                        if cube.mergeable(other) {
                            connection.join(cube, other);
                        }
                    }
                }
            }
        }

        for group in connection.groups() {
            let mut arena = Arena::new(self);
            for &index in group.iter() {
                if !arena.input(index) {
                    break;
                }
            }
            if matches!(arena.output(), ArenaResult::Draw) {
                group.into_iter().for_each(|i| self.cube[i].balanced = true);
            }
        }

        for index in undetermined {
            if self.cube[index].balanced {
                impact.insert(index);
            }
        }

        self.connect(impact, &successors, Constraint::Slap, &mut connection)
            .into_iter()
            .for_each(|index| self.cube[index].constraint = Constraint::Slap);
    }

    fn connect(
        &self,
        determined: impl IntoIterator<Item = usize>,
        successors: &Digraph,
        constraint: Constraint,
        connection: &mut DisjointSet,
    ) -> HashSet<usize> {
        let number_of_cubes = self.cube.len();
        let mut visit = HashSet::with_capacity(number_of_cubes);
        let mut queue = VecDeque::with_capacity(number_of_cubes);
        for index in determined.into_iter() {
            let cube = &self.cube[index];
            if cube.constraint <= constraint && visit.insert(index) {
                queue.push_back(cube);
            }

            while let Some(precursor) = queue.pop_front() {
                for successor in successors
                    .children(precursor)
                    .map(|&index| &self.cube[index])
                    .filter(|cube| cube.constraint <= constraint)
                {
                    if visit.insert(successor.index) {
                        queue.push_back(successor);
                    }
                    if precursor.linkable(successor) {
                        connection.join(precursor, successor);
                    }
                }
            }
        }

        visit
    }

    fn link(&mut self, connection: &mut DisjointSet) {
        for group in connection.groups() {
            let mut arena = Arena::new(self);
            for &index in group.iter() {
                arena.input(index);
            }
            if let ArenaResult::Pure(kind) = arena.output() {
                self.merge(group, kind);
            }
        }
    }

    fn merge(&mut self, from: Vec<usize>, kind: Kind) {
        let cube = &mut self.cube;

        let units = {
            let capacity = from.iter().map(|&i| cube[i].units.len()).sum::<usize>();
            let mut units = Vec::with_capacity(capacity);
            for &i in from.iter() {
                units.append(&mut cube[i].units);
            }
            let collision = HashSetCollision::new(units.iter().map(|unit| unit.position));
            for unit in units.iter_mut() {
                unit.neighborhood = collision.neighborhood(unit.position);
            }
            units
        };
        let motion = {
            let mut others = Vec::with_capacity(from.len());
            for &i in from.iter() {
                if cube[i].kind == kind {
                    others.push(Motion::take(&mut cube[i].motion));
                }
            }
            Motion::from_iter(others.into_iter())
        };
        let contours = Contours::new(&units).into();
        let movement = {
            let mut agreement = Agreement::new();
            for &i in from.iter() {
                if cube[i].kind == kind {
                    agreement.submit(cube[i].movement);
                    if agreement.fail() {
                        break;
                    }
                }
            }
            agreement.result().unwrap_or_default()
        };
        let constraint = {
            let mut constraint = Constraint::Free;
            for &i in from.iter() {
                if cube[i].kind == kind {
                    constraint = constraint.max(cube[i].constraint);
                }
            }
            constraint
        };

        if let Some(&index) = from.first() {
            cube[index] = Cube {
                index,
                kind,
                units,
                motion,
                contours,
                balanced: false,
                movement,
                constraint,
            };
        }
    }

    fn bury(&mut self) {
        let len = self.cube.len();
        self.cube.retain(Cube::alive);
        if len != self.cube.len() {
            self.cube
                .iter_mut()
                .enumerate()
                .rev()
                .take_while(|(index, cube)| *index != cube.index)
                .for_each(|(index, cube)| cube.index = index);
        }
    }
}

/////////////////////////////////////////////////////////////////////////////
// internal

#[derive(Clone, Debug)]
struct Cube {
    index: usize,
    kind: Kind,
    units: Vec<Unit>,
    motion: Motion,
    contours: Rc<Contours>,     // calculated boundary points
    balanced: bool,             // state of being unabsorbable
    movement: Option<Movement>, // original movement direction
    constraint: Constraint,     // state of movement
}

impl Cube {
    fn alive(&self) -> bool {
        !self.units.is_empty()
    }

    const fn unstable(&self) -> bool {
        !self.balanced && !matches!(self.kind, Kind::White)
    }

    const fn linkable(&self, other: &Self) -> bool {
        self.kind.linkable(other.kind)
    }

    const fn absorbable(&self, other: &Self) -> bool {
        !self.balanced && !other.balanced && self.kind.absorbable(other.kind)
    }

    const fn mergeable(&self, other: &Self) -> bool {
        self.kind.linkable(other.kind)
            || ((self.kind.absorbable(other.kind) || other.kind.absorbable(self.kind))
                && !self.balanced
                && !other.balanced)
    }

    fn same_movement(&self, other: &Self) -> bool {
        self.movement == other.movement
    }
}

impl std::hash::Hash for Cube {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.index.hash(state);
    }
}

impl PartialEq for Cube {
    fn eq(&self, other: &Self) -> bool {
        self.index == other.index
    }
}

impl Eq for Cube {}

impl From<&Cube> for usize {
    fn from(it: &Cube) -> Self {
        it.index
    }
}

#[derive(Clone, Debug)]
struct Unit {
    index: usize,
    position: Point,
    neighborhood: Neighborhood,
}

impl Unit {
    fn is_border(&self) -> bool {
        !self.neighborhood.contains(&Neighborhood::CROSS)
    }
}

#[derive(Debug)]
pub struct ImmutableArea {
    unchanged: Box<[(Point, Neighborhood)]>,
    collision: BitmapCollision,
}

impl ImmutableArea {
    pub fn new<'a, I>(width: usize, height: usize, it: I) -> Self
    where
        I: Iterator<Item = &'a [Point]>,
    {
        let cubes = {
            let build = |os: &'a [Point]| {
                let c = HashSetCollision::new(os.iter());
                os.iter().map(move |&o| (o, c.neighborhood(o)))
            };
            it.flat_map(build).collect::<Box<_>>()
        };

        let collision = {
            let mut it = BitmapCollision::new(width, height);
            cubes.iter().for_each(|x| it.put(x.0));
            it
        };

        Self {
            unchanged: cubes,
            collision,
        }
    }

    pub fn blocked(&self, point: Point) -> bool {
        self.collision.hit(point)
    }

    pub fn iter(&self, offset: usize) -> AreaIter {
        AreaIter {
            offset,
            iterator: self.unchanged.iter().enumerate(),
        }
    }
}

pub struct AreaIter<'a> {
    offset: usize,
    iterator: std::iter::Enumerate<std::slice::Iter<'a, (Point, Neighborhood)>>,
}

impl Iterator for AreaIter<'_> {
    type Item = Item;

    fn next(&mut self) -> Option<Self::Item> {
        self.iterator
            .next()
            .map(|(index, (point, neighborhood))| Item {
                id: (index + self.offset).into(),
                kind: Kind::White,
                position: point.clone(),
                movement: None,
                constraint: Constraint::Free,
                neighborhood: neighborhood.clone(),
            })
    }
}

#[derive(Debug)]
struct Contours {
    count: [usize; 3],
    slice: Box<[Point]>,
}

impl Contours {
    fn new(units: &[Unit]) -> Self {
        const LBTR: [Adjacence; 4] = [
            Adjacence::LEFT,
            Adjacence::BOTTOM,
            Adjacence::TOP,
            Adjacence::RIGHT,
        ];

        // count:
        // [  0     1       2    3   ]
        // 0..left..bottom..top..right
        let mut count: [usize; 4] = Default::default();
        for unit in units.iter() {
            for (i, a) in LBTR.into_iter().enumerate() {
                if !unit.neighborhood.has(a) {
                    count[i] += 1;
                }
            }
        }
        for i in 1..count.len() {
            count[i] += count[i - 1];
        }

        let mut slice = {
            let first = Self::anchor(units);
            let total = count[3];
            vec![first; total]
        };
        let mut index = {
            let mut index = count.clone();
            index.rotate_right(1);
            index[0] = 0;
            index
        };
        for unit in units.iter() {
            for (i, a) in LBTR.into_iter().enumerate() {
                if !unit.neighborhood.has(a) {
                    let point = &mut slice[index[i]];
                    *point -= unit.position;
                    *point -= a.into();
                    index[i] += 1;
                }
            }
        }

        Self {
            count: [count[0], count[1], count[2]],
            slice: slice.into(),
        }
    }

    fn one(&self, anchor: Point, m: Movement) -> impl Iterator<Item = Point> + Clone + '_ {
        use Movement::*;
        match m {
            Left /***/ => &self.slice[ /*      **/ ..self.count[0]],
            Down /***/ => &self.slice[self.count[0]..self.count[1]],
            Up /*  **/ => &self.slice[self.count[1]..self.count[2]],
            Right /**/ => &self.slice[self.count[2].. /*      **/ ],
        }
        .iter()
        .map(move |o| anchor - *o)
    }

    fn all(&self, anchor: Point) -> impl Iterator<Item = Point> + Clone + '_ {
        (&self.slice[..]).iter().map(move |o| anchor - *o)
    }

    fn anchor(units: &[Unit]) -> Point {
        units.first().map(|u| u.position).unwrap_or_default()
    }
}

/////////////////////////////////////////////////////////////////////////////
// additional lookups

struct Territory<'a>(HashMap<Point, &'a Cube>);

impl<'a> Territory<'a> {
    fn new<I, C>(it: I) -> Self
    where
        I: Iterator<Item = C> + Clone,
        C: Into<&'a Cube>,
    {
        let mut capacity = 0;
        for cube in it.clone().map(Into::into) {
            for _ in cube.units.iter().filter(|unit| unit.is_border()) {
                capacity += 1;
            }
        }

        let mut map = HashMap::with_capacity(capacity);
        for cube in it.map(Into::into) {
            for unit in cube.units.iter().filter(|unit| unit.is_border()) {
                map.insert(unit.position, cube);
            }
        }

        Self(map)
    }

    fn get(&self, point: Point) -> Option<&Cube> {
        self.0.get(&point).cloned()
    }

    fn neighbors(&self, cube: impl Into<&'a Cube>) -> impl Iterator<Item = &Cube> + Clone + '_ {
        let cube: &'a Cube = cube.into();
        let anchor = Contours::anchor(&cube.units);
        cube.contours.all(anchor).filter_map(|o| self.get(o))
    }

    fn neighbors_in_front(&self, cube: &Moving<'a>) -> impl Iterator<Item = &Cube> + Clone + '_ {
        cube.frontlines().filter_map(|o| self.get(o))
    }
}

struct QuarterTerritory<'a>(HashMap<Point, &'a Cube>);

impl<'a> QuarterTerritory<'a> {
    fn new<I, C>(it: I) -> Self
    where
        I: Iterator<Item = C> + Clone,
        C: Into<&'a Cube>,
    {
        let mut capacity = 0;
        for cube in it.clone().map(Into::into) {
            for _ in cube.units.iter().filter(|unit| unit.is_border()) {
                capacity += 4;
            }
        }

        let mut map = HashMap::with_capacity(capacity);
        for cube in it.clone().map(Into::into) {
            let cube: &Cube = cube;
            let delta = Self::delta(cube);
            for unit in cube.units.iter().filter(|unit| unit.is_border()) {
                let point = unit.position * 2 + delta;
                map.insert(point + Point::new(0, 0), cube);
                map.insert(point + Point::new(0, 1), cube);
                map.insert(point + Point::new(1, 0), cube);
                map.insert(point + Point::new(1, 1), cube);
            }
        }

        Self(map)
    }

    fn neighbors(&self, cube: impl Into<&'a Cube>) -> impl Iterator<Item = &Cube> + Clone + '_ {
        let cube: &'a Cube = cube.into();
        let delta = Self::delta(cube);
        let anchor = Contours::anchor(&cube.units);
        Movement::ALL.into_iter().flat_map(move |movement| {
            cube.contours
                .one(anchor, movement)
                .flat_map(move |mut point| {
                    point *= 2;
                    point += delta;
                    match movement {
                    Movement::Left /* **/ => [Point::new(1, 0), Point::new(1, 1)],
                    Movement::Down /* **/ => [Point::new(0, 0), Point::new(1, 0)],
                    Movement::Up /*   **/ => [Point::new(0, 1), Point::new(1, 1)],
                    Movement::Right /***/ => [Point::new(0, 0), Point::new(0, 1)],
                }
                    .map(|x| point + x)
                    .into_iter()
                    .filter_map(|point| self.0.get(&point))
                    .cloned()
                })
        })
    }

    fn delta(cube: &'a Cube) -> Point {
        match cube.movement {
            Some(movement) if cube.constraint <= Constraint::Slap => movement.into(),
            _ => Point::new(0, 0),
        }
    }
}

#[derive(Default)]
struct Conflict(HashMap<Point, [Option<usize>; 4]>);

impl Conflict {
    fn with_capacity(capacity: usize) -> Self {
        Self(HashMap::with_capacity(capacity))
    }

    fn put<T, I>(&mut self, index: T, movement: Movement, contours: I)
    where
        T: Into<usize>,
        I: Iterator<Item = Point>,
    {
        let value = index.into();
        use Movement::*;
        let index = match movement {
            Left => 0,
            Down => 1,
            Up => 3,
            Right => 2,
        };
        contours.for_each(|point| self.0.entry(point).or_default()[index] = Some(value));
    }

    fn overlaps(self) -> HashSet<[Option<usize>; 4]> {
        self.0
            .into_values()
            .filter(|race| race.iter().cloned().filter(Option::is_some).take(2).count() == 2)
            .collect()
    }

    const fn locked(cube: &[Cube], this: usize, other: Option<usize>) -> bool {
        match other {
            Some(other) => !cube[this].absorbable(&cube[other]),
            None /*__*/ => false,
        }
    }
}

struct Arena<'a>([bool; 3], &'a Collection);

enum ArenaResult {
    Have(Kind),
    Draw,
    Pure(Kind),
    None,
}

impl<'a> Arena<'a> {
    fn new(collection: &'a Collection) -> Self {
        Self([false; 3], collection)
    }

    fn input(&mut self, i: usize) -> bool {
        // "try" expression is experimental :-(
        if let Some(index) = self.1.cube.get(i).and_then(|x| Self::kind_to_index(x.kind)) {
            self.0[index] = true;
        };

        !(self.0[0] && self.0[1] && self.0[2])
    }

    fn output(&self) -> ArenaResult {
        use ArenaResult::*;
        use Kind::*;
        match ((self.0[0] as usize) << 2)
            | ((self.0[1] as usize) << 1)
            | ((self.0[2] as usize) << 0)
        {
            0b111 => Draw,
            0b101 => Have(Red),
            0b110 => Have(Blue),
            0b011 => Have(Green),
            0b001 => Pure(Red),
            0b010 => Pure(Blue),
            0b100 => Pure(Green),
            _else => None,
        }
    }

    const fn kind_to_index(kind: Kind) -> Option<usize> {
        use Kind::*;
        match kind {
            White => None,
            Red => Some(0),
            Blue => Some(1),
            Green => Some(2),
        }
    }
}

/////////////////////////////////////////////////////////////////////////////
// Moving

#[derive(Debug)]
struct Moving<'a> {
    cube: &'a Cube,
    movement: Movement,
}

impl<'a> Moving<'a> {
    fn new(cube: &'a Cube) -> Option<Self> {
        if !cube.alive() {
            None
        } else if let Some(movement) = cube.movement {
            Some(Self { cube, movement })
        } else {
            None
        }
    }

    fn frontlines(&self) -> impl Iterator<Item = Point> + Clone + 'a {
        let movement = self.movement;
        let anchor = Contours::anchor(&self.cube.units);
        self.cube.contours.one(anchor, movement)
    }
}

impl From<&Moving<'_>> for usize {
    fn from(it: &Moving) -> Self {
        it.cube.index
    }
}

impl<'a> From<&Moving<'a>> for &'a Cube {
    fn from(it: &Moving<'a>) -> Self {
        it.cube
    }
}

impl<'a> From<Moving<'a>> for &'a Cube {
    fn from(it: Moving<'a>) -> Self {
        it.cube
    }
}

impl<'a> std::ops::Deref for Moving<'a> {
    type Target = Cube;

    fn deref(&self) -> &'a Self::Target {
        self.cube
    }
}

/////////////////////////////////////////////////////////////////////////////
// tests

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn contours() {
        let contours = Contours::new(&[]);
        let actual = Vec::from_iter(contours.all(Point::new(0, 0)));
        assert_eq!(actual, Vec::new());
        for movement in Movement::ALL {
            let actual = Vec::from_iter(contours.one(Point::new(0, 0), movement));
            assert_eq!(actual, Vec::new());
        }

        let units = vec![
            Unit {
                index: 0,
                position: Point::new(0, 0),
                neighborhood: Neighborhood::from([Adjacence::BOTTOM].into_iter()),
            },
            Unit {
                index: 0,
                position: Point::new(0, 1),
                neighborhood: Neighborhood::from([Adjacence::TOP].into_iter()),
            },
        ];
        let contours = Contours::new(&units);

        let expected = vec![Point::new(0, 1), Point::new(0, 2)];
        let actual = Vec::from_iter(contours.one(Point::new(1, 1), Movement::Left));
        assert_eq!(actual, expected);

        let expected = vec![Point::new(2, 1), Point::new(2, 2)];
        let actual = Vec::from_iter(contours.one(Point::new(1, 1), Movement::Right));
        assert_eq!(actual, expected);

        let expected = vec![Point::new(1, 0)];
        let actual = Vec::from_iter(contours.one(Point::new(1, 1), Movement::Up));
        assert_eq!(actual, expected);

        let expected = vec![Point::new(1, 3)];
        let actual = Vec::from_iter(contours.one(Point::new(1, 1), Movement::Down));
        assert_eq!(actual, expected);
    }
}
