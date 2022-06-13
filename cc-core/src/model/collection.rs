use std::iter;

use super::{Behavior, Collision, Key, Movement, Type};
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

#[derive(Clone)]
struct Head {
    // necessary
    kind: Type,
    units: Vec<UnitID>,
    behavior: Option<Behavior>,
    // temporary
    edges: Option<Borders>,
}

#[derive(Clone)]
pub struct Unit {
    head: HeadID,
    position: Point,
    neighborhood: Neighborhood,
}

impl From<&Unit> for Key {
    fn from(o: &Unit) -> Self {
        Self::from(&o.position)
    }
}

#[derive(Clone)]
pub struct Collection {
    heads: Vec<Head>,
    units: Box<[Unit]>,
}

impl Collection {
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
            this.1
                .units
                .reserve(they.iter().map(|that| that.1.units.len()).sum());
            this.1
                .units
                .extend(they.iter_mut().map(|that| that.1.units.drain(..)).flatten());
            this.1.units.sort();
            pick(self.units.iter_mut(), this.1.units.iter()).for_each(|u| u.head = this.0.clone());
            update_neighborhood(pick(self.units.iter_mut(), this.1.units.iter()));

            // [2] reconstruct behavior
            this.1.behavior = Behavior::from_iter(
                iter::once(this.1.behavior.take()).chain(
                    they.iter_mut()
                        .filter(|that| that.1.kind.absorbable(this.1.kind))
                        .map(|that| that.1.behavior.take()),
                ),
            );

            // [3] reconstruct edges.
            they.iter_mut().for_each(|o| o.1.edges = None);
            this.1.edges = Some(Borders::new(
                pick_indexed(
                    self.units.iter_mut(),
                    this.1.units.iter().map(|u| u.clone()),
                )
                .map(|(i, u)| (i, u.neighborhood)),
            ));
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

        marks.iter().rev().for_each(|&i| {
            let _ = self.heads.swap_remove(i);
        });

        let limit = self.heads.len();
        pick_indexed(
            self.heads.iter_mut(),
            marks.into_iter().filter(move |&i| i < limit),
        )
        .for_each(|(i, head)| {
            pick(self.units.iter_mut(), head.units.iter()).for_each(|u| u.head = i.into())
        });
    }

    // fn update_headid(&mut self, x: &mut Head) {
    //     pick(self.units.iter_mut(), x.units.iter()).for_each(|u| u.head = i.into());
    // }
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
    data: Vec<UnitID>,
}

impl Borders {
    pub fn new<'a, T>(it: T) -> Self
    where
        T: Iterator<Item = (UnitID, Neighborhood)>,
    {
        let v = it.collect::<Vec<_>>();
        let mut data = Vec::with_capacity(v.len() * 4);
        let mut size: [usize; 3] = [0, 0, 0];

        const RIGHT: Adjacence = Adjacence::RIGHT;
        const NOT_RIGHT: [Adjacence; 3] = [Adjacence::LEFT, Adjacence::BOTTOM, Adjacence::TOP];
        for (i, a) in NOT_RIGHT.into_iter().enumerate() {
            data.extend(v.iter().filter(|o| !o.1.has(a)).map(|o| o.0.clone()));
            size[i] = data.len();
        }
        data.extend(v.into_iter().filter(|o| !o.1.has(RIGHT)).map(|o| o.0));
        data.shrink_to_fit();

        Self { size, data }
    }

    pub fn get(&self, m: Movement) -> &[UnitID] {
        match m {
            Movement::Idle => self.data.as_slice(),
            Movement::Left => &self.data[0..self.size[0]],
            Movement::Down => &self.data[self.size[0]..self.size[1]],
            Movement::Up => &self.data[self.size[1]..self.size[2]],
            Movement::Right => &self.data[self.size[2]..],
        }
    }
}

fn update_neighborhood<'a, I>(it: I)
where
    I: Iterator<Item = &'a mut Unit>,
{
    let us = it.collect::<Vec<_>>();
    let co = Collision::new(us.iter().map(|u| u.position));
    us.into_iter().for_each(|u| {
        u.neighborhood = Neighborhood::from(
            Neighborhood::AROUND
                .into_iter()
                .filter(|o| co.hit(u.position.near(*o))),
        )
    });
}

fn pick<'a, I, T, U, V>(mut it: I, is: U) -> impl Iterator<Item = &'a mut T>
where
    I: Iterator<Item = &'a mut T> + 'a,
    T: 'a,
    U: Iterator<Item = V>,
    V: Into<usize>,
{
    let mut o: Option<usize> = None;
    is.map(|x| x.into())
        .filter_map(move |x| match o {
            None => {
                let x = x;
                o = Some(x);
                Some(x)
            }
            Some(k) if x > k => {
                o = Some(x);
                Some(x - k)
            }
            Some(_) => None,
        })
        .filter_map(move |n| it.nth(n))
}

fn pick_indexed<'a, I, T, U, V>(mut it: I, is: U) -> impl Iterator<Item = (V, &'a mut T)>
where
    I: Iterator<Item = &'a mut T>,
    T: 'a,
    U: Iterator<Item = V>,
    V: Into<usize> + From<usize>, // or replace From<usize> with clone
{
    let mut o: Option<usize> = None;
    is.map(|x| x.into())
        .filter_map(move |x| match o {
            None => {
                o = Some(x);
                Some((x, x))
            }
            Some(k) if x > k => {
                o = Some(x);
                Some((x, x - k))
            }
            Some(_) => None,
        })
        .filter_map(move |(i, n)| it.nth(n).map(|x| (i.into(), x)))
}
