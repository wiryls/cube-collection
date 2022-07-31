use std::{
    collections::{HashMap, HashSet, VecDeque},
    rc::Rc,
};

use super::{
    extension::CollisionExtension,
    item::Item,
    kind::Kind,
    lookup::{self, BitmapCollision, Collision, Digraph, HashSetCollision},
    motion::{Agreement, Motion},
    movement::{Constraint, Movement},
    neighborhood::{Adjacence, Neighborhood},
    point::Point,
};

/////////////////////////////////////////////////////////////////////////////
// export

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct Collection {
    area: Rc<ImmutableArea>, // background and obstacles
    cube: Vec<Cube>,         // cubes (sets of units)
}

#[allow(dead_code)]
impl Collection {
    pub fn next(&mut self) {
        // clean status
        // move to next status
        todo!()
    }

    pub fn input(&mut self, input: Option<Movement>) {
        for cube in self.cube.iter_mut().filter(|cube| cube.kind == Kind::Green) {
            cube.movement = input;
        }
    }

    pub fn bury(&mut self) {
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

    pub fn absorb(&mut self) {
        let number_of_cubes = self.number_of_cubes();
        let unstable = self.cube.iter().filter(|u| u.alive() && u.unstable());

        let territory = Territory::new(unstable.clone());
        let mut connection = Connection::new(number_of_cubes);

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
                Have(kind) => self.link_into_first(group, kind),
                Draw => self.mark_balanced(group),
                _ => {}
            }
        }
    }

    fn perform_stopping(&mut self) -> Digraph {
        let number_of_cubes = self.number_of_cubes();
        let faction = Territory::new(self.cube.iter().filter(|cube| cube.alive()));
        let mut determined = Vec::new();
        let mut connection = Connection::new(number_of_cubes);
        let mut successors = Digraph::with_capacity(number_of_cubes);

        // find explicit blocked cubes.
        for cube in self.cube.iter().filter_map(Moving::new) {
            let mut blocked = cube.frontlines().any(|o| self.area.blocked(o));

            if !blocked {
                let neighbors = faction.neighbors_in_front(&cube).collect::<HashSet<_>>();
                blocked = neighbors
                    .iter()
                    .any(|&other| !cube.same_movement(other) && !cube.linkable(other));

                if !blocked {
                    for &other in neighbors.iter() {
                        if !cube.same_movement(other) && cube.linkable(other) {
                            blocked = true;
                            determined.push(other);
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
                determined.push(cube.into());
            }
        }

        // collect them and try to connect.
        let mut visit = HashSet::with_capacity(number_of_cubes);
        let mut queue = VecDeque::with_capacity(number_of_cubes);
        for cube in determined {
            if visit.insert(cube.index) {
                queue.push_back(cube);
            }

            while let Some(owner) = queue.pop_front() {
                for child in successors.children(owner).map(|&index| &self.cube[index]) {
                    if visit.insert(child.index) {
                        queue.push_back(child);
                    }
                    if owner.linkable(child) {
                        connection.join(owner, child);
                    }
                }
            }
        }

        // try to absorb each others.
        for group in connection.groups() {
            let mut arena = Arena::new(self);
            for &index in group.iter() {
                arena.input(index);
            }
            if let ArenaResult::Pure(kind) = arena.output() {
                self.link_into_first(group, kind);
            }
        }

        // mark all visited as stopped.
        for index in visit {
            self.cube[index].constraint = Constraint::Stop;
        }

        // reuse it!
        successors
    }

    fn perform_locking(&mut self, successors: Digraph) {
        let number_of_cubes = self.number_of_cubes();
        /* number_of_cubes is inaccurate but enough */
        let mut conflict = Conflict::with_capacity(number_of_cubes);

        self.cube
            .iter()
            .filter_map(Moving::new)
            .filter(|cube| cube.constraint < Constraint::Stop)
            .for_each(|cube| conflict.put(&cube, cube.movement, cube.frontlines()));

        let races = conflict.overlaps();
        for race in races.iter() {
            race.solve(&mut self.cube);
        }
        let undetermined = races
            .into_iter()
            .filter_map(|race| race.free(&self.cube))
            .collect::<Vec<_>>();
    }

    fn mark_balanced(&mut self, whom: Vec<usize>) {
        for index in whom {
            self.cube[index].balanced = true;
        }
    }

    fn link_into_first(&mut self, from: Vec<usize>, kind: Kind) {
        let cube = &mut self.cube;

        let units = {
            let capacity = from.iter().map(|&i| cube[i].units.len()).sum::<usize>();
            let mut units = Vec::with_capacity(capacity);
            for &i in from.iter() {
                units.append(&mut cube[i].units);
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

    fn number_of_cubes(&self) -> usize {
        self.cube.len()
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

    const fn linkable(&self, that: &Self) -> bool {
        self.kind.linkable(that.kind)
    }

    const fn absorbable(&self, that: &Self) -> bool {
        !self.balanced && !that.balanced && self.kind.absorbable(that.kind)
    }

    const fn mergeable(&self, that: &Self) -> bool {
        self.kind.linkable(that.kind)
            || ((self.kind.absorbable(that.kind) || that.kind.absorbable(self.kind))
                && !self.balanced
                && !that.balanced)
    }

    fn same_movement(&self, that: &Self) -> bool {
        self.movement == that.movement
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

#[allow(dead_code)]
#[derive(Clone, Debug)]
struct Unit {
    index: usize,
    position: Point,
    neighborhood: Neighborhood,
}

#[derive(Debug)]
pub struct ImmutableArea {
    unchanged: Box<[(Point, Neighborhood)]>,
    collision: BitmapCollision,
}

#[allow(dead_code)]
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

#[allow(dead_code)]
#[derive(Debug)]
struct Contours {
    count: [usize; 3],
    slice: Box<[Point]>,
}

#[allow(dead_code)]
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

type Connection = lookup::DisjointSet;

struct Territory<'a>(HashMap<Point, &'a Cube>);

impl<'a> Territory<'a> {
    fn new<I, C>(it: I) -> Self
    where
        I: Iterator<Item = C> + Clone,
        C: Into<&'a Cube>,
    {
        let size = it.clone().map(|c| c.into().units.len()).sum::<usize>();
        let mut map = HashMap::with_capacity(size);
        for cube in it.map(Into::into) {
            for unit in cube.units.iter() {
                if !unit.neighborhood.contains(&Neighborhood::CROSS) {
                    map.insert(unit.position, cube);
                }
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

#[derive(Default)]
struct Conflict(HashMap<Point, Race>);

impl Conflict {
    fn with_capacity(capacity: usize) -> Self {
        Self(HashMap::with_capacity(capacity))
    }

    fn put<T, I>(&mut self, index: T, movement: Movement, contours: I)
    where
        T: Into<usize>,
        I: Iterator<Item = Point>,
    {
        let index = index.into();
        contours.for_each(|point| self.0.entry(point).or_default().put(movement, index));
    }

    fn overlaps(self) -> HashSet<Race> {
        self.0.into_values().filter(Race::conflict).collect()
    }
}

#[derive(Default, PartialEq, Eq, Hash)]
struct Race([Option<usize>; 4]);

impl Race {
    fn put(&mut self, movement: Movement, index: usize) {
        self.0[Self::movement_to_index(movement)] = Some(index);
    }

    fn conflict(&self) -> bool {
        self.0.into_iter().filter(Option::is_some).take(2).count() == 2
    }

    fn solve(&self, cube: &mut [Cube]) {
        let size = self.0.len();
        let half = size >> 1;
        for i in 0..self.0.len() {
            let it = match self.0[i] {
                Some(index) if cube[index].constraint < Constraint::Lock => index,
                ________________________________________________________ => continue,
            };

            let last = self.0[(i + size - 1) % size];
            let next = self.0[(i + 0000 + 1) % size];
            if Self::is_locked(cube, it, last) || Self::is_locked(cube, it, next) {
                cube[it].constraint = Constraint::Lock;
                continue;
            }

            let oppo = self.0[(i + half) % size];
            if Self::is_locked(cube, it, oppo) {
                cube[it].constraint = Constraint::Pong;
            }
        }
    }

    fn free(&self, cube: &[Cube]) -> Option<usize> {
        for index in self.0 {
            if let Some(index) = index {
                if cube[index].constraint == Constraint::Free {
                    return Some(index);
                }
            }
        }
        None
    }

    const fn is_locked(cube: &[Cube], this: usize, that: Option<usize>) -> bool {
        match that {
            Some(that) => !cube[this].absorbable(&cube[that]),
            None /*_*/ => false,
        }
    }

    const fn movement_to_index(movement: Movement) -> usize {
        use Movement::*;
        match movement {
            Left => 0,
            Down => 1,
            Up => 3,
            Right => 2,
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
