use super::{Borders, Collision, DisjointSet, HeadID, Motion, Movement, Restriction, Type, UnitID};
use crate::common::{self, Neighborhood, Point};

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

#[derive(Clone)]
struct Head {
    // necessary
    kind: Type,
    units: Box<[UnitID]>, // sorted
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

impl Collection {
    pub fn next(&self, merge: DisjointSet, action: Option<Restrictions>) -> Self {
        let mut units = self.units.clone();
        let heads = self
            .mappings(merge)
            .map(|mapping| {
                let (index, mut head) = match mapping.source {
                    Source::Copy { head } => (head.index, head.refer.clone()),
                    Source::Link { head, tail } => {
                        let units = Self::link_units(&head, &tail);
                        let motion = Self::link_motions(&head, &tail);
                        let movement = motion.get();
                        let borders = Self::link_boarders(&units, &self.units);
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
                    Self::wind_up(&mut head, &mut units);
                }

                // TODO: update headid of units.

                head
            })
            .collect::<Box<_>>();

        Self { heads, units }
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

    fn wind_up(x: &mut Head, units: &mut Box<[Unit]>) {
        use {super::Movable, Movement::Idle, Restriction::Free};
        if matches!(x.restrict, Free) && !matches!(x.movement, Idle) {
            for u in pick(units.iter_mut(), x.units.iter()) {
                u.position.step(x.movement);
            }
        }
    }

    fn link_units(head: &IndexedHead, tail: &[IndexedHead]) -> Box<[UnitID]> {
        let size = head.refer.units.len() + tail.iter().map(|o| o.refer.units.len()).sum::<usize>();
        let mut list = Vec::with_capacity(size);
        list.extend_from_slice(&head.refer.units);
        tail.iter()
            .for_each(|o| list.extend_from_slice(&o.refer.units));
        list.sort();
        list.into_boxed_slice()
    }

    fn link_motions(head: &IndexedHead, tail: &[IndexedHead]) -> Motion {
        Motion::from_iter(
            std::iter::once(&head.refer.motion).chain(
                tail.iter()
                    .filter(|o| o.refer.kind.absorbable(head.refer.kind))
                    .map(|o| &o.refer.motion),
            ),
        )
    }

    fn link_boarders(indexes: &[UnitID], units: &[Unit]) -> Borders {
        Borders::new(
            indexes
                .iter()
                .map(usize::from)
                .filter_map(|i| units.get(i).map(|u| (i.into(), u.neighborhood))),
        )
    }
}

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
//// utilities

trait MutableUnitsExtension {
    fn update_head(&mut self, i: HeadID);
    fn update_neighborhood(&mut self);
}

impl MutableUnitsExtension for [&mut Unit] {
    fn update_head(&mut self, i: HeadID) {
        self.iter_mut().for_each(|u| u.head = i.clone());
    }

    fn update_neighborhood(&mut self) {
        let co = Collision::new(self.iter().map(|u| u.position));
        self.iter_mut().for_each(|u| {
            use common::Adjacent;
            u.neighborhood = Neighborhood::from(
                Neighborhood::AROUND
                    .into_iter()
                    .filter(|o| co.hit(u.position.near(*o))),
            )
        });
    }
}

fn pick<'a, I, T, U, V>(mut it: I, is: U) -> impl Iterator<Item = &'a mut T>
where
    I: Iterator<Item = &'a mut T> + 'a,
    T: 'a,
    U: Iterator<Item = V>,
    V: Into<usize>,
{
    let mut last = 0;
    let mut init = true;
    let monotonic_increasing = move |next| {
        if next > last {
            let step = next - last - 1;
            last = next;
            Some(step)
        } else if last == 0 && init {
            init = false;
            Some(0)
        } else {
            None
        }
    };

    // ignore non-monotonic-increasing numbers.
    is.map(|x| x.into())
        .filter_map(monotonic_increasing)
        .filter_map(move |n| it.nth(n))
}

fn pick_indexed<'a, I, T, U, V>(mut it: I, is: U) -> impl Iterator<Item = (V, &'a mut T)>
where
    I: Iterator<Item = &'a mut T>,
    T: 'a,
    U: Iterator<Item = V>,
    V: Into<usize> + From<usize>,
{
    let mut last = 0;
    let mut init = true;
    let monotonic_increasing = move |next| {
        if next > last {
            let step = next - last - 1;
            last = next;
            Some((next, step))
        } else if last == 0 && init {
            init = false;
            Some((0, 0))
        } else {
            None
        }
    };

    // ignore non-monotonic-increasing numbers.
    is.map(|x| x.into())
        .filter_map(monotonic_increasing)
        .filter_map(move |(i, n)| it.nth(n).map(|x| (i.into(), x)))
}
