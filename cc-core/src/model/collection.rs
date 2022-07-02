use std::rc::Rc;

use super::{Collision, DisjointSet, HeadID, Motion, Movement, Restriction, Type, UnitID};
use crate::{
    common::{Adjacence, Neighborhood, Point},
    Faction,
};

#[derive(Clone)]
pub struct Collection {
    heads: Box<[Head]>,
    units: Box<[Unit]>,
    cache: Cache,
}

impl Collection {
    pub fn new() -> Self {
        todo!()
    }

    pub fn number_of_cubes(&self) -> usize {
        self.heads.len()
    }

    pub fn number_of_units(&self) -> usize {
        self.units.len()
    }

    pub fn cube(&self, index: HeadID) -> CollectedCube<'_> {
        CollectedCube::new(self, index)
    }

    pub fn cubes(&self) -> impl Iterator<Item = CollectedCube<'_>> + Clone {
        (0..self.heads.len()).map(|i| CollectedCube::new(self, i.into()))
    }

    pub fn transform(&self, merge: DisjointSet, action: Option<&[Restriction]>) -> Self {
        let mut cache = self.cache.clone();
        let mut units = self.units.clone();
        let heads = self
            .mappings(merge)
            .map(|mapping| {
                let mut rebind = mapping.moving;
                let mut moveon = false;
                let mut reunit = false;

                let (index, mut head) = match mapping.source {
                    Source::Copy { head } => (head.index, head.refer.clone()),
                    Source::Link { head, tail } => {
                        rebind = true;
                        reunit = true;

                        let units = Self::make_units(&head, &tail);
                        let motion = Self::make_motions(&head, &tail);
                        let movement = motion.get();
                        let outlines = Outlines::new(&self.units, &units);
                        let copy = Head {
                            kind: head.refer.kind.clone(),
                            units,
                            motion,
                            movement,
                            restrict: head.refer.restrict.clone(),
                            outlines,
                        };

                        (head.index, copy)
                    }
                };

                let i = usize::from(&index);
                if let Some(&restrict) = action.and_then(|r| r.get(i)) {
                    head.movement = head.motion.get();
                    head.restrict = restrict;
                    head.motion.next();

                    use {Movement::Idle, Restriction::Free};
                    moveon = matches!(restrict, Free) && !matches!(head.movement, Idle);
                }

                if rebind || moveon || reunit {
                    let point = head.movement.into();
                    for i in head.units.iter() {
                        let mut unit = &mut units[usize::from(i)];
                        if rebind {
                            unit.head = index.clone();
                        }
                        if moveon {
                            unit.position += point;
                        }
                    }

                    if reunit {
                        let c = Collision::new(units.iter().map(|u| u.position));
                        for i in head.units.iter() {
                            let mut unit = &mut units[usize::from(i)];
                            unit.neighborhood = Neighborhood::from(
                                Neighborhood::AROUND
                                    .into_iter()
                                    .filter(|&o| c.hit(unit.position + o.into())),
                            )
                        }
                    }

                    if moveon {
                        cache.faction = Rc::new(Self::make_faction(&units));
                    }
                }

                head
            })
            .collect::<Box<_>>();

        Self {
            heads,
            units,
            cache,
        }
    }

    pub fn differ(&self, _that: &Self) /* -> ? */
    {
        todo!()
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

#[derive(Clone)]
pub struct CollectedCube<'a> {
    head: &'a Head,
    owner: &'a Collection,
    index: HeadID,
}

impl<'a> CollectedCube<'a> {
    fn new(owner: &'a Collection, index: HeadID) -> Self {
        Self {
            head: &owner.heads[usize::from(&index)],
            owner,
            index,
        }
    }

    pub fn id(&self) -> HeadID {
        self.index.clone()
    }

    pub fn index(&self) -> usize {
        self.index.clone().into()
    }

    pub fn kind(&self) -> Type {
        self.head.kind
    }

    pub fn unstable(&self) -> bool {
        self.head.kind != Type::White
    }

    pub fn absorbable(&self, that: &Self) -> bool {
        self.head.kind.absorbable(that.head.kind)
    }

    pub fn absorbable_actively(&self, that: &Self) -> bool {
        self.head.kind.absorbable_actively(that.head.kind)
    }

    pub fn movement(&self) -> Movement {
        self.head.movement
    }

    pub fn moving(&self) -> bool {
        self.head.movement != Movement::Idle
    }

    pub fn outlines_ahead(&self) -> impl Iterator<Item = Point> + Clone + 'a {
        let anchor = Outlines::anchor(self.head.units.first(), &self.owner.units);
        self.head.outlines.out(anchor, self.head.movement)
    }

    pub fn outlines(&self) -> impl Iterator<Item = Point> + Clone + 'a {
        let anchor = Outlines::anchor(self.head.units.first(), &self.owner.units);
        self.head.outlines.out(anchor, Movement::Idle)
    }

    pub fn neighbors_ahead(&self) -> impl Iterator<Item = CollectedCube<'a>> + Clone {
        let faction = &self.owner.cache.faction;
        self.outlines_ahead()
            .filter_map(|o| faction.get(o).map(|i| Self::new(self.owner, i)))
    }

    pub fn neighbors(&self) -> impl Iterator<Item = CollectedCube<'a>> + Clone {
        let faction = &self.owner.cache.faction;
        self.outlines()
            .filter_map(|o| faction.get(o).map(|i| Self::new(self.owner, i)))
    }
}

impl From<&CollectedCube<'_>> for usize {
    fn from(that: &CollectedCube<'_>) -> Self {
        that.index()
    }
}

impl PartialEq for CollectedCube<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.index == other.index
    }
}

impl Eq for CollectedCube<'_> {}

/////////////////////////////////////////////////////////////////////////////
// subtypes

#[derive(Clone)]
struct Head {
    // necessary
    kind: Type,
    units: Box<[UnitID]>,
    motion: Motion,
    // calculated
    movement: Movement,
    restrict: Restriction,
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
                    *point += a.into();
                    *point -= unit.position;
                    index[i] += 1;
                }
            }
        }

        Self {
            count: [count[0], count[1], count[2]],
            slice: slice.into(),
        }
    }

    pub fn out<'a>(
        &'a self,
        anchor: Point,
        action: Movement,
    ) -> impl Iterator<Item = Point> + Clone + 'a {
        match action {
            Movement::Idle => &self.slice[..],
            Movement::Left => &self.slice[..self.count[0]],
            Movement::Down => &self.slice[self.count[0]..self.count[1]],
            Movement::Up => &self.slice[self.count[1]..self.count[2]],
            Movement::Right => &self.slice[self.count[2]..],
        }
        .iter()
        .map(move |o| anchor - *o)
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
