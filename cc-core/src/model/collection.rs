use std::iter;

use super::{Behavior, Collision, DisjointSet, Key, Movement, Type};
use crate::common::{Adjacence, Adjacent, Neighborhood, Point};

#[derive(Clone, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub struct HeadID(usize);

impl From<usize> for HeadID {
    fn from(i: usize) -> Self {
        Self(i)
    }
}
impl From<HeadID> for usize {
    fn from(i: HeadID) -> Self {
        i.0
    }
}
impl From<&HeadID> for usize {
    fn from(i: &HeadID) -> Self {
        i.0
    }
}

#[derive(Clone, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub struct UnitID(usize);

impl From<usize> for UnitID {
    fn from(i: usize) -> Self {
        Self(i)
    }
}
impl From<UnitID> for usize {
    fn from(i: UnitID) -> Self {
        i.0
    }
}
impl From<&UnitID> for usize {
    fn from(i: &UnitID) -> Self {
        i.0
    }
}

pub struct Background(Box<[(Point, Neighborhood)]>);

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
    behavior: Option<Behavior>,
    // calculated
    borders: Borders,
}

#[derive(Clone)]
struct Unit {
    head: HeadID,
    position: Point,
    neighborhood: Neighborhood,
}

pub enum Patch {
    Join(Vec<Vec<HeadID>>),
    // Move(Box<[(HeadID, Status)]>),
}

impl Collection {
    pub fn next(join: DisjointSet /*, order: Order*/) {}

    // pub fn head(&self, id: &HeadID) -> Option<&Head> {
    //     self.heads.get(id)
    // }

    // pub fn heads(&self) -> impl Iterator<Item = (HeadID, &Head)> {
    //     self.heads.iter().enumerate().map(|x| (x.0.into(), x.1))
    // }

    // pub fn unit(&self, id: &UnitID) -> Option<&Unit> {
    //     self.units.get(id.0)
    // }

    // pub fn groups<'a, P>(
    //     &'a self,
    //     filter: P,
    // ) -> impl Iterator<Item = (HeadID, &Head, impl Iterator<Item = &Unit>)>
    // where
    //     P: Fn(&Head) -> bool + 'a,
    // {
    //     self.heads
    //         .iter()
    //         .enumerate()
    //         .filter(move |x| filter(x.1))
    //         .map(|x| {
    //             (
    //                 x.0.into(),
    //                 x.1,
    //                 x.1.units.iter().filter_map(|x| self.units.get(x.0)),
    //             )
    //         })
    // }

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

// impl Head {
//     fn unstable(&self) -> bool {
//         self.kind.unstable()
//     }

//     fn absorbable(&self, that: &Self) -> bool {
//         self.kind.absorbable(&that.kind)
//     }

//     fn absorbable_actively(&self, that: &Self) -> bool {
//         self.kind.absorbable_actively(&that.kind)
//     }

//     fn edges(&self, m: Movement) -> &[UnitID] {
//         const EMPTY: [UnitID; 0] = [];
//         self.edges
//             .as_ref()
//             .map(|x| x.get(m))
//             .unwrap_or(EMPTY.as_slice())
//     }
// }

#[derive(Clone)]
pub struct Borders {
    size: [usize; 3],
    list: Box<[UnitID]>,
}

impl Borders {
    pub fn new<'a, T>(it: T) -> Self
    where
        T: Iterator<Item = (UnitID, Neighborhood)>,
    {
        let v = it.collect::<Vec<_>>();
        let mut list = Vec::with_capacity(v.len() * 4);
        let mut size: [usize; 3] = [0, 0, 0];

        const RIGHT: Adjacence = Adjacence::RIGHT;
        const NOT_RIGHT: [Adjacence; 3] = [Adjacence::LEFT, Adjacence::BOTTOM, Adjacence::TOP];
        for (i, a) in NOT_RIGHT.into_iter().enumerate() {
            list.extend(v.iter().filter(|o| !o.1.has(a)).map(|o| o.0.clone()));
            size[i] = list.len();
        }
        list.extend(v.into_iter().filter(|o| !o.1.has(RIGHT)).map(|o| o.0));

        let list = list.into();
        Self { size, list }
    }

    pub fn get(&self, m: Movement) -> &[UnitID] {
        match m {
            Movement::Idle => &self.list,
            Movement::Left => &self.list[..self.size[0]],
            Movement::Down => &self.list[self.size[0]..self.size[1]],
            Movement::Up => &self.list[self.size[1]..self.size[2]],
            Movement::Right => &self.list[self.size[2]..],
        }
    }
}

trait WritableUnitsExtension {
    fn update_head(&mut self, i: HeadID);
    fn update_neighborhood(&mut self);
}

impl WritableUnitsExtension for [&mut Unit] {
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
    is.map(|i| i.into())
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
