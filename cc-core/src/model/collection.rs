use super::{Collision, DisjointSet, HeadID, Motion, Movement, Restriction, Type, UnitID};
use crate::common::{self, Adjacence, Neighborhood, Point};

pub struct Restrictions(Box<[Restriction]>);

impl Restrictions {
    pub fn new(collection: &Collection) -> Self {
        Self(collection.heads.iter().map(|h| h.restrict).collect())
    }
}

#[derive(Clone)]
pub struct Collection {
    heads: Box<[Head]>,
    units: Box<[Unit]>,
}

impl Collection {
    pub fn new() -> Self {
        todo!()
    }

    pub fn next(&self, merge: DisjointSet, action: Option<Restrictions>) -> Self {
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
                        let borders = Borders::new(&self.units, &units);
                        let copy = Head {
                            kind: head.refer.kind.clone(),
                            units,
                            motion,
                            movement,
                            restrict: head.refer.restrict.clone(),
                            borders,
                        };

                        (head.index, copy)
                    }
                };

                let i = usize::from(&index);
                if let Some(&restrict) = action.as_ref().and_then(|r| r.0.get(i)) {
                    head.movement = head.motion.get();
                    head.restrict = restrict;
                    head.motion.next();

                    use {Movement::Idle, Restriction::Free};
                    moveon = matches!(restrict, Free) && !matches!(head.movement, Idle);
                }

                if rebind || moveon || reunit {
                    let reunit = reunit.then(|| Collision::new(units.iter().map(|u| u.position)));

                    for i in head.units.iter() {
                        let mut unit = &mut units[usize::from(i)];
                        if rebind {
                            unit.head = index.clone();
                        }
                        if moveon {
                            use super::Movable;
                            unit.position.step(head.movement);
                        }
                        if let Some(c) = &reunit {
                            use common::Adjacent;
                            unit.neighborhood = Neighborhood::from(
                                Neighborhood::AROUND
                                    .into_iter()
                                    .filter(|&o| c.hit(unit.position.near(o))),
                            )
                        }
                    }
                }

                head
            })
            .collect::<Box<_>>();

        Self { heads, units }
    }

    pub fn diff(&self, that: &Self) /* -> ? */
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

        for group in set.groups() {
            let mut group = group
                .into_iter()
                .map(usize::from)
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
}

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
    borders: Borders,
}

#[derive(Clone)]
struct Unit {
    head: HeadID,
    position: Point,
    neighborhood: Neighborhood,
}

#[derive(Clone)]
struct Borders {
    count: [usize; 4],
    slice: Box<[UnitID]>,
}

impl Borders {
    pub fn new(units: &[Unit], indexes: &[UnitID]) -> Self {
        fn loop_through(units: &[Unit], indexes: &[UnitID], mut f: impl FnMut(UnitID, usize)) {
            const LBTR: [Adjacence; 4] = [
                Adjacence::LEFT,
                Adjacence::BOTTOM,
                Adjacence::TOP,
                Adjacence::RIGHT,
            ];

            for id in indexes.iter() {
                let mut found = false;
                let unit = &units[usize::from(id)];
                for (i, a) in LBTR.into_iter().enumerate() {
                    if !unit.neighborhood.has(a) {
                        found = true;
                        f(id.clone(), i + 1 /* [1, 5) */);
                    }
                }
                if found {
                    f(id.clone(), 0);
                }
            }
        }

        let mut count: [usize; 5] = Default::default();
        loop_through(units, indexes, |_, i| count[i] += 1);
        let total = count.iter().sum::<usize>();
        for i in 1..count.len() {
            // [  0     1       2    3      4 ]
            // 0..left..bottom..top..right..len
            count[i] += count[i - 1];
        }

        let mut index = count.clone();
        index.rotate_right(1);
        index[0] = 0;

        let mut slice = vec![UnitID::from(0); total];
        loop_through(units, indexes, |u, i| {
            let j = index[i];
            slice[j] = u;
            index[i] += 1;
        });

        let count = [count[0], count[1], count[2], count[3]];
        let slice = slice.into();
        Self { count, slice }
    }

    pub fn get(&self, m: Movement) -> &[UnitID] {
        match m {
            Movement::Idle => &self.slice[..self.count[0]],
            Movement::Left => &self.slice[self.count[0]..self.count[1]],
            Movement::Down => &self.slice[self.count[1]..self.count[2]],
            Movement::Up => &self.slice[self.count[2]..self.count[3]],
            Movement::Right => &self.slice[self.count[3]..],
        }
    }
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
