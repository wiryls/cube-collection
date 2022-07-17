use std::{borrow::Borrow, collections::VecDeque, rc::Rc};

use super::{
    extension::CollisionExtension,
    item::Item,
    kind::Kind,
    lookup::{BitmapCollision, Collision, HashSetCollision},
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
    area: Rc<Area>,
    sets: Vec<Cube>,
    todo: Vec<Vec<usize>>,
    view: Box<[Unit]>,
}

#[allow(dead_code)]
impl Collection {
    pub fn absorb(&mut self) -> &mut Self {
        let unstable = (0..self.sets.len())
            .filter_map(|index| Living::make(index, self).filter(Living::unstable))
            .collect::<Vec<_>>();

        let faction = Faction::new::<Living, _, _>(self, unstable.iter());
        let mut connect = Connection::new(self);

        let mut visit = vec![false; self.number_of_cubes()];
        let mut queue = VecDeque::with_capacity(unstable.len());

        // connect all adjacent cubes.
        for cube in unstable {
            if !visit[cube.index] {
                visit[cube.index] = true;
                queue.push_back(cube);
            }

            while let Some(cube) = queue.pop_back() {
                for other in faction.neighbors(&cube) {
                    if !visit[other.index] {
                        connect.join(&cube, &other);
                        visit[other.index] = true;
                        queue.push_back(other);
                    }
                }
            }
        }

        // try to absorb each others.
        for group in connect.groups() {
            let mut arena = Arena::new(self);
            for &index in group.iter() {
                if !arena.input(index) {
                    break;
                }
            }
            use ArenaResult::*;
            match arena.output() {
                Know(kind) => self.merge_into(group, kind),
                Draw => self.mark_balanced(group),
                None => {}
            }
        }

        // do some cleaning.
        self.retain_alive();
        self
    }

    pub fn input(&mut self, input: Option<Movement>) {
        // clear and
        // update movement with input.
        self.todo.clear();
        for cube in self.sets.iter_mut().filter(|cube| cube.kind == Kind::Green) {
            cube.movement = input;
        }

        // parpare
        let unstable = (0..self.sets.len())
            .filter_map(|index| Living::make(index, self).filter(Living::unstable))
            .collect::<Vec<_>>();

        let moving = (0..self.sets.len()).filter_map(|index| Moving::make(index, self));

        todo!()
    }

    pub fn next(&mut self) {
        // clean status
        // move to next status
        todo!()
    }

    fn retain_alive(&mut self) {
        self.sets.retain(Cube::alive);
    }

    fn merge_into(&mut self, from: Vec<usize>, into: Kind) {
        let cube = &mut self.sets;

        let units = {
            let size = from.iter().map(|&i| cube[i].units.len()).sum::<usize>();
            let mut units = Vec::with_capacity(size);
            for &i in from.iter() {
                units.append(&mut cube[i].units);
            }
            units
        };
        let motion = {
            let mut others = Vec::with_capacity(from.len());
            for &i in from.iter() {
                if cube[i].kind == into {
                    others.push(Motion::take(&mut cube[i].motion));
                }
            }
            Motion::from_iter(others.into_iter())
        };
        let outlines = Outlines::new(&units).into();
        let movement = {
            let mut agreement = Agreement::new();
            for &i in from.iter() {
                if cube[i].kind == into {
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
                if cube[i].kind == into {
                    constraint = constraint.max(cube[i].constraint);
                }
            }
            constraint
        };

        let cube = Cube {
            kind: into,
            units,
            motion,
            outlines,
            balanced: false,
            movement,
            constraint,
        };

        self.sets.push(cube);
    }

    fn mark_balanced(&mut self, whom: Vec<usize>) {
        for index in whom {
            self.sets[index].balanced = true;
        }
    }

    fn number_of_cubes(&self) -> usize {
        self.sets.len()
    }
}

/////////////////////////////////////////////////////////////////////////////
// internal

#[derive(Clone, Debug)]
struct Cube {
    kind: Kind,
    units: Vec<Unit>,
    motion: Motion,
    outlines: Rc<Outlines>,     // calculated boundary points
    balanced: bool,             // state of being unabsorbable
    movement: Option<Movement>, // original movement direction
    constraint: Constraint,     // state of movement
}

impl Cube {
    fn alive(&self) -> bool {
        !self.units.is_empty()
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
pub struct Area {
    cubes: Box<[(Point, Neighborhood)]>,
    impassable: BitmapCollision,
}

#[allow(dead_code)]
impl Area {
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

        let impassable = {
            let mut it = BitmapCollision::new(width, height);
            cubes.iter().for_each(|x| it.put(x.0));
            it
        };

        Self { cubes, impassable }
    }

    pub fn blocked(&self, point: Point) -> bool {
        self.impassable.hit(point)
    }

    pub fn iter(&self, offset: usize) -> AreaIter {
        AreaIter {
            offset,
            iterator: self.cubes.iter().enumerate(),
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
struct Outlines {
    count: [usize; 3],
    slice: Box<[Point]>,
}

#[allow(dead_code)]
impl Outlines {
    pub fn new(units: &[Unit]) -> Self {
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

    pub fn one(&self, anchor: Point, m: Movement) -> impl Iterator<Item = Point> + Clone + '_ {
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

    pub fn all(&self, anchor: Point) -> impl Iterator<Item = Point> + Clone + '_ {
        (&self.slice[..]).iter().map(move |o| anchor - *o)
    }

    fn anchor(units: &[Unit]) -> Point {
        units.first().map(|u| u.position).unwrap_or_default()
    }
}

/////////////////////////////////////////////////////////////////////////////
// lookups

struct Faction<'a>(super::lookup::Faction, &'a Collection);

impl<'a> Faction<'a> {
    fn new<C, B, I>(collection: &'a Collection, it: I) -> Self
    where
        I: Iterator<Item = B>,
        B: Borrow<C>,
        C: Cubic<'a>,
    {
        let faction = super::lookup::Faction::new(it.flat_map(|b| {
            let cubic = b.borrow();
            let index = cubic.index();
            let value = cubic.value();
            value.units.iter().map(move |unit| (unit.position, index))
        }));
        Self(faction, collection)
    }

    fn get(&self, point: Point) -> Option<Living<'a>> {
        self.0
            .get(point)
            .and_then(|index| Living::make(index, self.1))
    }

    fn neighbors<T>(&self, cube: &T) -> impl Iterator<Item = Living> + Clone + '_
    where
        T: Cubic<'a>,
    {
        let cube = cube.value();
        let anchor = Outlines::anchor(&cube.units);
        cube.outlines.all(anchor).filter_map(|o| self.get(o))
    }

    fn neighbors_in_front(&self, cube: &Moving<'a>) -> impl Iterator<Item = Living> + Clone + '_ {
        cube.frontlines().filter_map(|o| self.get(o))
    }
}

struct Connection<'a>(super::lookup::DisjointSet, &'a Collection);

impl<'a> Connection<'a> {
    fn new(collection: &'a Collection) -> Self {
        let number = collection.number_of_cubes();
        Self(super::lookup::DisjointSet::new(number), collection)
    }

    fn join<'b, L: Cubic<'b>, R: Cubic<'b>>(&mut self, this: &'b L, that: &'b R) {
        self.0.join(this.index(), that.index());
    }

    fn groups(self) -> super::lookup::DisjointSetGroups {
        self.0.groups()
    }
}

struct Arena<'a>([bool; 3], &'a Collection);

enum ArenaResult {
    Know(Kind),
    Draw,
    None,
}

impl<'a> Arena<'a> {
    fn new(collection: &'a Collection) -> Self {
        Self([false; 3], collection)
    }

    fn input(&mut self, i: usize) -> bool {
        if let Some(index) = self.1.sets.get(i).and_then(|x| Self::kind_to_index(x.kind)) {
            self.0[index] = true;
        };

        !(self.0[0] && self.0[1] && self.0[2])
    }

    fn output(&self) -> ArenaResult {
        use ArenaResult::*;
        use Kind::*;
        match (self.0[0], self.0[1], self.0[2]) {
            (true, true, true) => Draw,
            (true, true, false) => Know(Blue),
            (true, false, true) => Know(Red),
            (false, true, true) => Know(Green),
            _ => None,
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
// Collected

trait Cubic<'a> {
    fn index(&self) -> usize;
    fn value(&self) -> &'a Cube;
    fn owner(&self) -> &'a Collection;
}

impl<'a> Cubic<'a> for Living<'a> {
    fn index(&self) -> usize {
        self.index
    }

    fn value(&self) -> &'a Cube {
        self.value
    }

    fn owner(&self) -> &'a Collection {
        self.owner
    }
}

trait CubicExtension {
    fn kind(&self) -> Kind;
    fn unstable(&self) -> bool;
    fn linkable<T: CubicExtension>(&self, that: &T) -> bool;
    fn absorbable<T: CubicExtension>(&self, that: &T) -> bool;
}

impl<'a, T: Cubic<'a>> CubicExtension for T {
    fn kind(&self) -> Kind {
        self.value().kind
    }

    fn unstable(&self) -> bool {
        let cube = self.value();
        !cube.balanced && cube.kind != Kind::White
    }

    fn linkable<U: CubicExtension>(&self, that: &U) -> bool {
        self.kind().linkable(that.kind())
    }

    fn absorbable<U: CubicExtension>(&self, that: &U) -> bool {
        self.unstable() && that.unstable() && self.kind().absorbable(that.kind())
    }
}

#[derive(Debug)]
struct Living<'a> {
    index: usize,
    value: &'a Cube,
    owner: &'a Collection,
}

impl<'a> Living<'a> {
    fn make(index: usize, owner: &'a Collection) -> Option<Self> {
        owner
            .sets
            .get(index)
            .filter(|cube| cube.alive())
            .map(|value| Self {
                value,
                index,
                owner,
            })
    }

    fn into_moving(self) -> Option<Moving<'a>> {
        self.value.movement.map(|movement| Moving {
            index: self.index,
            value: self.value,
            owner: self.owner,
            movement,
        })
    }
}

#[derive(Debug)]
struct Moving<'a> {
    index: usize,
    value: &'a Cube,
    owner: &'a Collection,
    movement: Movement,
}

impl<'a> Moving<'a> {
    fn make(index: usize, owner: &'a Collection) -> Option<Self> {
        owner
            .sets
            .get(index)
            .filter(|cube| cube.movement.is_some() && cube.alive())
            .map(|value| Self {
                value,
                index,
                owner,
                movement: value.movement.unwrap(),
            })
    }

    fn frontlines(&self) -> impl Iterator<Item = Point> + Clone + 'a {
        let movement = self.movement;
        let anchor = Outlines::anchor(&self.value.units);
        self.value.outlines.one(anchor, movement)
    }
}

/////////////////////////////////////////////////////////////////////////////
// tests

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn outlines() {
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
        let outlines = Outlines::new(&units);

        let expected = vec![Point::new(0, 1), Point::new(0, 2)];
        let actual = Vec::from_iter(outlines.one(Point::new(1, 1), Movement::Left));
        assert_eq!(actual, expected);

        let expected = vec![Point::new(2, 1), Point::new(2, 2)];
        let actual = Vec::from_iter(outlines.one(Point::new(1, 1), Movement::Right));
        assert_eq!(actual, expected);

        let expected = vec![Point::new(1, 0)];
        let actual = Vec::from_iter(outlines.one(Point::new(1, 1), Movement::Up));
        assert_eq!(actual, expected);

        let expected = vec![Point::new(1, 3)];
        let actual = Vec::from_iter(outlines.one(Point::new(1, 1), Movement::Down));
        assert_eq!(actual, expected);
    }
}
