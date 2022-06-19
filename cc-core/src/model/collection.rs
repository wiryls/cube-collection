use std::{convert::identity, iter, ops::Index};

use super::{Behavior, Borders, Collision, DisjointSet, HeadID, Restriction, Type, UnitID};
use crate::common::{Adjacent, Neighborhood, Point};

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
    units: Box<[UnitID]>,
    behavior: Behavior,
    // calculated
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
    pub fn next(&self, set: DisjointSet, res: Option<Restrictions>) {
        enum Mapping {
            Copy(HeadID),
            Link(HeadID, Box<[HeadID]>),
        }

        let mut mapping = (0..self.heads.len())
            .map(Into::into)
            .map(Mapping::Copy)
            .map(Some)
            .collect::<Box<_>>();

        for (main, side) in set.groups().filter_map(|group| self.pick(group)) {
            for i in side.iter() {
                if let Some(m) = mapping.get_mut(usize::from(i)) {
                    *m = None;
                }
            }
            if let Some(m) = mapping.get_mut(usize::from(&main)) {
                *m = Some(Mapping::Link(main, side));
            }
        }

        let mut units = self.units.clone();
        let mut heads = Vec::from(mapping)
            .into_iter()
            .filter_map(identity)
            .map(|x| match x {
                Mapping::Copy(_) => todo!(),
                Mapping::Link(_, _) => todo!(),
            });

        todo!()
    }

    fn pick(&self, group: Vec<HeadID>) -> Option<(HeadID, Box<[HeadID]>)> {
        let mut group = group
            .into_iter()
            .filter_map(|i| self.heads.get(usize::from(i.clone())).map(|x| (i, x)))
            .collect::<Vec<_>>();

        group
            .iter()
            .position(|this| group.iter().all(|that| this.1.kind.absorbable(that.1.kind)))
            .map(|one| {
                let head = group.swap_remove(one).0;
                let tail = group.into_iter().map(|x| x.0).collect();
                (head, tail)
            })
    }

    pub fn merge(&mut self, heads: impl Iterator<Item = HeadID>) {
        let mut they = pick_indexed(self.heads.iter_mut(), heads).collect::<Vec<_>>();

        if let Some(winner) = they
            .iter()
            .position(|this| they.iter().all(|that| this.1.kind.absorbable(that.1.kind)))
        {
            let mut this = they.swap_remove(winner);

            // [1] reconstruct units
            // - move they.units into this.units, and then sort
            // - update units.head and units.neighborhood
            let unitids = &mut this.1.units;
            // unitids.reserve(they.iter().map(|that| that.1.units.len()).sum());
            // unitids.extend(they.iter_mut().map(|that| that.1.units.drain(..)).flatten());
            unitids.sort();

            let mut units: Vec<_> = pick(self.units.iter_mut(), unitids.iter()).collect();
            units.update_head(this.0);
            units.update_neighborhood();

            // [2] reconstruct behavior
            this.1.behavior = Behavior::from_iter(
                iter::once(this.1.behavior.take()).chain(
                    they.iter_mut()
                        .filter(|that| that.1.kind.absorbable(this.1.kind))
                        .map(|that| that.1.behavior.take()),
                ),
            );

            // [3] reconstruct edges.
            this.1.borders = Borders::new(
                read_indexed(&self.units, unitids.iter().cloned())
                    .map(|(i, u)| (i, u.neighborhood)),
            );
        }
    }

    pub fn clean(&mut self) {
        let marks = self
            .heads
            .iter()
            .enumerate()
            .filter(|x| x.1.units.is_empty())
            .map(|x| x.0)
            .collect::<Vec<_>>();
        for i in marks.iter().rev() {
            // _ = self.heads.swap_remove(*i);
        }

        let limit = self.heads.len();
        let marks = marks.into_iter().filter(move |i| *i < limit);
        for (i, head) in pick_indexed(self.heads.iter_mut(), marks) {
            pick(self.units.iter_mut(), head.units.iter()).for_each(|u| u.head = i.into());
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
            u.neighborhood = Neighborhood::from(
                Neighborhood::AROUND
                    .into_iter()
                    .filter(|o| co.hit(u.position.near(*o))),
            )
        });
    }
}

fn read_indexed<'a, T, U, V>(it: &'a [T], is: U) -> impl Iterator<Item = (V, &'a T)>
where
    T: 'a,
    U: Iterator<Item = V>,
    V: Into<usize> + From<usize>,
{
    is.map(Into::into)
        .filter_map(|i| it.get(i).map(|x| (i.into(), x)))
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
