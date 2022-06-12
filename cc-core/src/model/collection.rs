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

impl From<&UnitID> for usize {
    fn from(i: &UnitID) -> Self {
        i.0
    }
}

#[derive(Clone)]
pub struct Collection {
    heads: Vec<Head>,
    units: Box<[Unit]>,
}

impl Collection {
    pub fn head(&self, id: &HeadID) -> Option<&Head> {
        self.heads.get(id.0)
    }

    pub fn heads(&self) -> impl Iterator<Item = (HeadID, &Head)> {
        self.heads.iter().enumerate().map(|x| (x.0.into(), x.1))
    }

    pub fn unit(&self, id: &UnitID) -> Option<&Unit> {
        self.units.get(id.0)
    }

    pub fn groups<'a, P>(
        &'a self,
        filter: P,
    ) -> impl Iterator<Item = (HeadID, &Head, impl Iterator<Item = &Unit>)>
    where
        P: Fn(&Head) -> bool + 'a,
    {
        self.heads
            .iter()
            .enumerate()
            .filter(move |x| filter(x.1))
            .map(|x| {
                (
                    x.0.into(),
                    x.1,
                    x.1.units.iter().filter_map(|x| self.units.get(x.0)),
                )
            })
    }

    pub fn merge(&mut self, group: Vec<HeadID>) {
        let mut heads = distill_with_index(self.heads.iter_mut(), group.into_iter())
            .map(|(i, head)| (HeadID::from(i), head))
            .collect::<Vec<_>>();

        if let Some(i) = heads
            .iter()
            .position(|head| heads.iter().all(|other| head.1.absorbable(other.1)))
        {
            let mut crystal = heads.swap_remove(i);

            // reconstruct units
            crystal
                .1
                .units
                .reserve_exact(heads.iter().map(|other| other.1.units.len()).sum());
            heads
                .iter_mut()
                .for_each(|other| crystal.1.units.extend(other.1.units.drain(..)));
            crystal.1.units.sort();
            distill(self.units.iter_mut(), crystal.1.units.iter())
                .for_each(|u| u.head = crystal.0.clone());
            reconstruct_neighborhood(distill(self.units.iter_mut(), crystal.1.units.iter()));

            // reconstruct behavior
            crystal.1.behavior = Behavior::from_options(
                iter::once(crystal.1.behavior.take()).chain(
                    heads
                        .iter_mut()
                        .filter(|other| other.1.absorbable(crystal.1))
                        .map(|other| other.1.behavior.take()),
                ),
            );

            // reconstruct edges.
            heads.iter_mut().for_each(|o| o.1.edges = None);
            crystal.1.edges = Some(Borders::new(
                distill_with_index(self.units.iter_mut(), crystal.1.units.iter())
                    .map(|(i, unit)| (i.into(), unit.neighborhood)),
            ));
        }
    }
}

#[derive(Clone)]
pub struct Head {
    // necessary
    pub kind: Type,
    pub units: Vec<UnitID>,
    pub behavior: Option<Behavior>,
    // temporary
    pub edges: Option<Borders>,
}

impl Head {
    pub fn unstable(&self) -> bool {
        self.kind.unstable()
    }

    pub fn absorbable(&self, that: &Self) -> bool {
        self.kind.absorbable(&that.kind)
    }

    pub fn absorbable_actively(&self, that: &Self) -> bool {
        self.kind.absorbable_actively(&that.kind)
    }
}

#[derive(Clone)]
pub struct Unit {
    pub head: HeadID,
    pub position: Point,
    pub neighborhood: Neighborhood,
}

impl From<&Unit> for Key {
    fn from(o: &Unit) -> Self {
        Self::from(&o.position)
    }
}

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

fn reconstruct_neighborhood<'a, I>(it: I)
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

fn distill<'a, I, T, U, V>(mut it: I, is: U) -> impl Iterator<Item = &'a mut T>
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
                o = Some(x.into());
                Some(x - k)
            }
            Some(_) => None,
        })
        .filter_map(move |n| it.nth(n))
}

fn distill_with_index<'a, I, T, U, V>(mut it: I, is: U) -> impl Iterator<Item = (usize, &'a mut T)>
where
    I: Iterator<Item = &'a mut T>,
    T: 'a,
    U: Iterator<Item = V>,
    V: Into<usize>,
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
        .filter_map(move |(i, n)| it.nth(n).map(|x| (i, x)))
}
