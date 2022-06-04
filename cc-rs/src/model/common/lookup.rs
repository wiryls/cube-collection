use super::location::Location;
use crate::model::behavior::Movement;
use std::collections::{HashMap, HashSet};

#[derive(Eq, Hash, PartialEq)]
pub struct Key(u64);

impl<T: Location<i32>> From<&T> for Key {
    fn from(o: &T) -> Self {
        Key(((o.x() as u64) << 32) | (o.y() as u64))
    }
}

pub struct Existence(HashSet<Key>);

impl Existence {
    pub fn new<T, U>(it: T) -> Self
    where
        T: Iterator<Item = U>,
        U: Into<Key>,
    {
        Self(it.map(|k| k.into()).collect())
    }

    pub fn has<T: Into<Key>>(&self, k: T) -> bool {
        self.0.contains(&k.into())
    }
}

#[derive(Default)]
pub struct Collision(HashMap<Key, usize>);

#[allow(dead_code)]
impl Collision {
    pub fn new<T, U>(it: T) -> Self
    where
        T: Iterator<Item = (U, usize)>,
        U: Into<Key>,
    {
        Self(it.map(|(k, v)| (k.into(), v)).collect())
    }

    pub fn put<T: Into<Key>>(&mut self, k: T, v: usize) {
        self.0.insert(k.into(), v);
    }

    pub fn get<T: Into<Key>>(&self, k: T) -> Option<usize> {
        self.0.get(&k.into()).cloned()
    }
}

#[derive(Default)]
pub struct Conflict(HashMap<Key, Race>);

#[allow(dead_code)]
impl Conflict {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn put<T, U>(&mut self, keys: T, movement: Movement, index: usize)
    where
        T: Iterator<Item = U>,
        U: Into<Key>,
    {
        keys.for_each(|key| self.0.entry(key.into()).or_default().set(movement, index));
    }

    pub fn overlaps(
        &self,
    ) -> impl Iterator<Item = impl Iterator<Item = (Movement, usize)> + '_> + '_ {
        self.0.values().filter(|x| x.conflict()).map(|x| x.whom())
    }
}

#[derive(Default)]
struct Race {
    mark: u8,
    data: [usize; 4],
}

impl Race {
    const MASK: [u8; 4] = [0b0001, 0b0010, 0b0100, 0b1000];
    const MOVE: [Movement; 4] = [
        Movement::Left,
        Movement::Down,
        Movement::Up,
        Movement::Right,
    ];

    fn set(&mut self, movement: Movement, index: usize) {
        let i = match movement {
            Movement::Idle => return,
            Movement::Left => 0,
            Movement::Down => 1,
            Movement::Up => 2,
            Movement::Right => 3,
        };
        self.mark |= Race::MASK[i];
        self.data[i] = index;
    }

    fn conflict(&self) -> bool {
        self.mark & self.mark - 1 != 0
    }

    fn whom(&self) -> impl Iterator<Item = (Movement, usize)> + '_ {
        (0..4)
            .into_iter()
            .filter(|i| self.mark & Race::MASK[*i] != 0)
            .map(|i| (Race::MOVE[i], self.data[i]))
    }
}
