use super::{Behavior, Key, Movement, Type};
use crate::common::{Adjacence, Neighborhood, Point};

#[derive(Clone, Eq, Hash, PartialEq)]
pub struct HeadID(usize);

impl From<usize> for HeadID {
    fn from(i: usize) -> Self {
        Self(i)
    }
}

impl From<&HeadID> for usize {
    fn from(i: &HeadID) -> Self {
        i.0
    }
}

#[derive(Clone)]
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
        T: Iterator<Item = (UnitID, Neighborhood)> + Clone,
    {
        let mut data = Vec::with_capacity(it.clone().count() * 4);
        let mut size: [usize; 3] = [0, 0, 0];

        const NOT_RIGHT: [Adjacence; 3] = [Adjacence::LEFT, Adjacence::BOTTOM, Adjacence::TOP];
        for (i, a) in NOT_RIGHT.into_iter().enumerate() {
            data.extend(it.clone().filter(|o| !o.1.has(a)).map(|o| o.0));
            size[i] = data.len();
        }
        data.extend(it.filter(|o| !o.1.has(Adjacence::RIGHT)).map(|o| o.0));
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
