use std::rc::Rc;

use super::{
    Action, Collision, DisjointSet, Faction, HeadID, Item, Kind, Motion, Movement, Restriction,
    UnitID,
};
use crate::common::{Adjacence, Neighborhood, Point};

#[derive(Clone)]
pub struct Collection {
    units: Box<[Unit]>,
    heads: Box<[Head]>,
    cache: Cache,
}

impl Collection {
    pub fn new<'a, I>(it: I) -> Self
    where
        I: Iterator<Item = (Kind, &'a [Point], Motion)> + 'a,
    {
        let mut units = Vec::new();
        let mut heads = Vec::new();
        for (kind, points, motion) in it {
            let offset = heads.len();
            let collision = Collision::new(points.iter().cloned());

            let begin = units.len();
            units.extend(points.iter().enumerate().map(|(i, &point)| Unit {
                head: HeadID::from(offset + i),
                position: point,
                neighborhood: collision.neighborhood(point),
            }));
            let end = units.len();

            let indexes = (begin..end).map(UnitID::from).collect::<Box<_>>();
            let outlines = Outlines::new(&units, &indexes);
            heads.push(Head {
                kind,
                units: indexes,
                motion,
                action: None,
                outlines,
            })
        }
        let cache = Cache {
            faction: Self::make_faction(units.as_slice()).into(),
        };

        Self {
            units: units.into(),
            heads: heads.into(),
            cache,
        }
    }

    pub fn iter(&self) -> CollectionIter {
        CollectionIter {
            collection: self,
            iterator: self.units.iter().enumerate(),
        }
    }

    pub fn number_of_cubes(&self) -> usize {
        self.heads.len()
    }

    pub fn number_of_units(&self) -> usize {
        self.units.len()
    }

    pub fn collected(&self, input: Option<Movement>) -> Collected {
        Collected {
            input,
            collection: self,
        }
    }

    pub fn exists(&self, point: Point) -> bool {
        self.cache.faction.get(point).is_some()
    }

    pub fn transform(&self, groups: DisjointSet, actions: Option<&[Option<Action>]>) -> Self {
        let mut remake_faction = false;
        let mut units = self.units.clone();
        let heads = self
            .mappings(groups)
            .map(|mapping| {
                let mut rebind = mapping.moving;
                let mut moveon = None;
                let mut reunit = false;

                let (index, mut head) = match mapping.source {
                    Source::Copy { head } => (head.index, head.refer.clone()),
                    Source::Link { head, tail } => {
                        rebind = true;
                        reunit = true;

                        let units = Self::make_units(&head, &tail);
                        let motion = Self::make_motions(&head, &tail);
                        let outlines = Outlines::new(&self.units, &units);
                        let copy = Head {
                            kind: head.refer.kind.clone(),
                            units,
                            motion,
                            action: head.refer.action.clone(),
                            outlines,
                        };

                        (head.index, copy)
                    }
                };

                if let Some(actions) = actions {
                    head.motion.next();
                    head.action = actions.get(usize::from(&index)).cloned().flatten();

                    if let Some(action) = &head.action {
                        if action.restriction == Restriction::Free {
                            moveon = Some(action.movement.into());
                        }
                    }
                }

                if rebind || moveon.is_some() || reunit {
                    for i in head.units.iter() {
                        let mut unit = &mut units[usize::from(i)];
                        if rebind {
                            unit.head = index.clone();
                        }
                        if let Some(point) = moveon {
                            unit.position += point;
                        }
                    }

                    if reunit {
                        let collision = Collision::new(
                            head.units.iter().map(|u| units[usize::from(u)].position),
                        );

                        for i in head.units.iter() {
                            let mut unit = &mut units[usize::from(i)];
                            unit.neighborhood = collision.neighborhood(unit.position);
                        }
                    }

                    remake_faction = true;
                }

                head
            })
            .collect::<Box<_>>();

        let cache = Cache {
            faction: if remake_faction {
                Rc::new(Self::make_faction(&units))
            } else {
                self.cache.faction.clone()
            },
        };

        Self {
            units,
            heads,
            cache,
        }
    }

    fn mappings(&self, set: DisjointSet) -> impl Iterator<Item = Mapping> {
        let mut mappings = self
            .heads
            .iter()
            .enumerate()
            .map(Mapping::from_copied)
            .collect::<Vec<_>>();

        for group in set.groups().iter() {
            let mut group = group
                .filter_map(|i| self.heads.get(i).map(|x| IndexedHead::new(i, x)))
                .collect::<Vec<_>>();

            if let Some((head, tail)) = group
                .iter()
                .position(|l| group.iter().all(|r| l.refer.kind.absorbable(r.refer.kind)))
                .map(|i| (group.swap_remove(i), group))
            {
                for i in tail.iter().map(|i| usize::from(&i.index)) {
                    mappings[i].moving = true; // a dummy mark
                }
                let i = usize::from(&head.index);
                mappings[i] = Mapping::from_linked(head, tail.into());
            }
        }

        let mut i = 0;
        let mut n = mappings.len();
        while i < n {
            if mappings[i].moving {
                n -= 1;
                while i < n && mappings[n].moving {
                    n -= 1;
                }
                if i < n {
                    mappings.swap(i, n);
                    mappings[i].moving = true; // mark as moved
                }
            }
            i += 1;
        }
        mappings.into_iter().take(n)
    }

    fn make_units(head: &IndexedHead, tail: &[IndexedHead]) -> Box<[UnitID]> {
        let size = head.refer.units.len() + tail.iter().map(|o| o.refer.units.len()).sum::<usize>();
        let mut list = Vec::with_capacity(size);
        list.extend_from_slice(&head.refer.units);
        tail.iter()
            .for_each(|o| list.extend_from_slice(&o.refer.units));
        list.into_boxed_slice()
    }

    fn make_motions(head: &IndexedHead, tail: &[IndexedHead]) -> Motion {
        Motion::from_iter(
            std::iter::once(&head.refer.motion).chain(
                tail.iter()
                    .filter(|o| o.refer.kind.absorbable(head.refer.kind))
                    .map(|o| &o.refer.motion),
            ),
        )
    }

    fn make_faction(units: &[Unit]) -> Faction {
        Faction::new(units.iter().map(|u| (u.position.into(), u.head.clone())))
    }
}

pub struct CollectionIter<'a> {
    collection: &'a Collection,
    iterator: std::iter::Enumerate<std::slice::Iter<'a, Unit>>,
}

impl Iterator for CollectionIter<'_> {
    type Item = Item;

    fn next(&mut self) -> Option<Self::Item> {
        self.iterator.next().map(|(i, u)| {
            let head = &self.collection.heads[usize::from(&u.head)];
            Item {
                id: i,
                kind: head.kind,
                action: head.action.clone(),
                position: u.position,
                neighborhood: u.neighborhood,
            }
        })
    }
}

#[derive(Clone)]
pub struct Collected<'a> {
    input: Option<Movement>,
    collection: &'a Collection,
}

impl Collected<'_> {
    pub fn cube(&self, index: HeadID) -> CollectedCube<'_> {
        CollectedCube::new(self, index)
    }

    pub fn cubes(&self) -> impl Iterator<Item = CollectedCube<'_>> + Clone {
        (0..self.collection.heads.len()).map(|i| CollectedCube::new(self, i.into()))
    }
}

trait Cubic {
    fn index(&self) -> &HeadID;
    fn value<'a>(&'a self) -> &'a Head;
    fn owner<'a>(&'a self) -> &'a Collected<'a>;
}

impl<T: Cubic> BasicCube for T {
    fn index(&self) -> usize {
        self.index().into()
    }

    fn id(&self) -> HeadID {
        self.index().clone()
    }

    fn kind(&self) -> Kind {
        self.value().kind
    }

    fn unstable(&self) -> bool {
        self.kind() != Kind::White
    }

    fn absorbable<U: BasicCube>(&self, that: &U) -> bool {
        self.kind().absorbable(that.kind())
    }

    fn absorbable_actively<U: BasicCube>(&self, that: &U) -> bool {
        self.kind().absorbable_actively(that.kind())
    }
}

pub trait BasicCube {
    fn index(&self) -> usize;
    fn id(&self) -> HeadID;
    fn kind(&self) -> Kind;
    fn unstable(&self) -> bool;
    fn absorbable<T: BasicCube>(&self, that: &T) -> bool;
    fn absorbable_actively<T: BasicCube>(&self, that: &T) -> bool;
}

#[derive(Clone)]
pub struct CollectedCube<'a> {
    owner: &'a Collected<'a>,
    value: &'a Head,
    index: HeadID,
}

impl Cubic for CollectedCube<'_> {
    fn index(&self) -> &HeadID {
        &self.index
    }

    fn value<'a>(&'a self) -> &'a Head {
        self.value
    }

    fn owner<'a>(&'a self) -> &'a Collected<'a> {
        self.owner
    }
}

impl From<&CollectedCube<'_>> for usize {
    fn from(that: &CollectedCube<'_>) -> Self {
        BasicCube::index(that)
    }
}

impl<'a> CollectedCube<'a> {
    fn new(owner: &'a Collected, index: HeadID) -> Self {
        Self {
            owner,
            value: &owner.collection.heads[usize::from(&index)],
            index,
        }
    }

    pub fn movable(&self) -> Option<Movement> {
        if self.owner.input.is_some() && self.value.kind == Kind::Blue {
            self.owner.input
        } else {
            self.value.motion.current()
        }
    }

    pub fn into_movable(self) -> Option<CollectedMovableCube<'a>> {
        self.movable().map(|movement| CollectedMovableCube {
            index: self.index,
            value: self.value,
            owner: self.owner,
            movement,
        })
    }

    pub fn outlines(&self) -> impl Iterator<Item = Point> + Clone + 'a {
        let anchor = Outlines::anchor(self.value.units.first(), &self.owner.collection.units);
        self.value.outlines.all(anchor)
    }

    pub fn neighbors(&self) -> impl Iterator<Item = CollectedCube<'a>> + Clone {
        let faction = &self.owner.collection.cache.faction;
        self.outlines()
            .filter_map(|o| faction.get(o).map(|i| Self::new(self.owner, i)))
    }
}

#[derive(Clone)]
pub struct CollectedMovableCube<'a> {
    index: HeadID,
    value: &'a Head,
    owner: &'a Collected<'a>,
    movement: Movement,
}

impl Cubic for CollectedMovableCube<'_> {
    fn index(&self) -> &HeadID {
        &self.index
    }

    fn value<'a>(&'a self) -> &'a Head {
        self.value
    }

    fn owner<'a>(&'a self) -> &'a Collected<'a> {
        self.owner
    }
}

impl From<&CollectedMovableCube<'_>> for usize {
    fn from(that: &CollectedMovableCube<'_>) -> Self {
        BasicCube::index(that)
    }
}

impl<'a> CollectedMovableCube<'a> {
    pub fn movement(&self) -> Movement {
        self.movement
    }

    pub fn outlines_in_front(&self) -> impl Iterator<Item = Point> + Clone + 'a {
        let movement = self.movement();
        let anchor = Outlines::anchor(self.value.units.first(), &self.owner.collection.units);
        self.value.outlines.one(anchor, movement)
    }

    pub fn neighbors_in_front(&self) -> impl Iterator<Item = CollectedCube<'a>> + Clone {
        let faction = &self.owner.collection.cache.faction;
        self.outlines_in_front()
            .filter_map(|o| faction.get(o).map(|i| CollectedCube::new(self.owner, i)))
    }
}

/////////////////////////////////////////////////////////////////////////////
// subtypes

#[derive(Clone)]
struct Head {
    // necessary
    kind: Kind,
    units: Box<[UnitID]>,
    motion: Motion,
    // calculated
    action: Option<Action>,
    outlines: Outlines,
}

#[derive(Clone)]
struct Unit {
    head: HeadID,
    position: Point,
    neighborhood: Neighborhood,
}

#[derive(Clone)]
struct Outlines {
    count: [usize; 3],
    slice: Box<[Point]>,
}

impl Outlines {
    pub fn new(units: &[Unit], indexes: &[UnitID]) -> Self {
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
        for id in indexes.iter() {
            let unit = &units[usize::from(id)];
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
            let first = Self::anchor(indexes.first(), units);
            let total = count[3];
            vec![first; total]
        };
        let mut index = {
            let mut index = count.clone();
            index.rotate_right(1);
            index[0] = 0;
            index
        };
        for id in indexes.iter() {
            let unit = &units[usize::from(id)];
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

    pub fn one<'a>(
        &'a self,
        anchor: Point,
        movement: Movement,
    ) -> impl Iterator<Item = Point> + Clone + 'a {
        match movement {
            Movement::Left => &self.slice[..self.count[0]],
            Movement::Down => &self.slice[self.count[0]..self.count[1]],
            Movement::Up => &self.slice[self.count[1]..self.count[2]],
            Movement::Right => &self.slice[self.count[2]..],
        }
        .iter()
        .map(move |o| anchor - *o)
    }

    pub fn all<'a>(&'a self, anchor: Point) -> impl Iterator<Item = Point> + Clone + 'a {
        (&self.slice[..]).iter().map(move |o| anchor - *o)
    }

    fn anchor(index: Option<&UnitID>, units: &[Unit]) -> Point {
        index
            .map(usize::from)
            .and_then(|i| units.get(i))
            .map(|u| u.position)
            .unwrap_or_default()
    }
}

#[derive(Clone)]
struct Cache {
    faction: Rc<Faction>,
}

/////////////////////////////////////////////////////////////////////////////
// intermediate types

struct IndexedHead<'a> {
    index: HeadID,
    refer: &'a Head,
}

impl<'a> IndexedHead<'a> {
    fn new<I: Into<HeadID>>(index: I, refer: &'a Head) -> Self {
        let index = index.into();
        Self { index, refer }
    }
}

enum Source<'a> {
    Copy {
        head: IndexedHead<'a>,
    },
    Link {
        head: IndexedHead<'a>,
        tail: Box<[IndexedHead<'a>]>,
    },
}

struct Mapping<'a> {
    moving: bool,
    source: Source<'a>,
}

impl<'a> Mapping<'a> {
    fn from_copied<I: Into<HeadID>>(pack: (I, &'a Head)) -> Self {
        let head = IndexedHead::new(pack.0.into(), pack.1);
        Self {
            moving: false,
            source: Source::Copy { head },
        }
    }

    fn from_linked(head: IndexedHead<'a>, tail: Box<[IndexedHead<'a>]>) -> Self {
        Self {
            moving: false,
            source: Source::Link { head, tail },
        }
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
                head: 0.into(),
                position: Point::new(0, 0),
                neighborhood: Neighborhood::from([Adjacence::BOTTOM].into_iter()),
            },
            Unit {
                head: 0.into(),
                position: Point::new(0, 1),
                neighborhood: Neighborhood::from([Adjacence::TOP].into_iter()),
            },
        ];
        let outlines = Outlines::new(&units, &[0, 1].map(UnitID::from));

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
