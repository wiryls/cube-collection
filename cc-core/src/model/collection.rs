use super::{Behavior, Movement, Type};
use crate::common::{Adjacence, Neighborhood, Point};

pub struct HeadID(usize);

pub struct UnitID(usize);

pub struct Collection {
    heads: Vec<Head>,
    units: Box<[Unit]>,
}

struct Head {
    // necessary
    kind: Type,
    units: Vec<UnitID>,
    behavior: Option<Behavior>,
    // temporary
    edges: Option<Borders>,
}

struct Unit {
    head: HeadID,
    position: Point,
    neighborhood: Neighborhood,
}

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
