use super::{Movement, UnitID};
use crate::common::{adjacence, Neighborhood};

pub struct Borders {
    size: [usize; 3],
    data: Vec<UnitID>,
}

impl Borders {
    pub fn new<'a, T>(it: T) -> Self
    where
        T: Iterator<Item = (UnitID, Neighborhood)> + Clone,
    {
        let mut size: [usize; 3] = [0, 0, 0];
        let mut data = Vec::with_capacity(it.clone().count() * 4);

        use adjacence::*;
        data.extend(it.clone().filter(|o| o.1.has(LEFT)).map(|o| o.0));
        size[0] = data.len();
        data.extend(it.clone().filter(|o| o.1.has(BOTTOM)).map(|o| o.0));
        size[1] = data.len();
        data.extend(it.clone().filter(|o| o.1.has(TOP)).map(|o| o.0));
        size[2] = data.len();
        data.extend(it.filter(|o| o.1.has(RIGHT)).map(|o| o.0));
        data.shrink_to_fit();

        Self { size, data }
    }

    fn get(&self, m: Movement) -> &[UnitID] {
        match m {
            Movement::Idle => self.data.as_slice(),
            Movement::Left => &self.data[0..self.size[0]],
            Movement::Down => &self.data[self.size[0]..self.size[1]],
            Movement::Up => &self.data[self.size[1]..self.size[2]],
            Movement::Right => &self.data[self.size[2]..],
        }
    }
}
