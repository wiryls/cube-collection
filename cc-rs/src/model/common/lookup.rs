use super::Point;
use crate::model::behavior::Movement;
use std::collections::{HashMap, HashSet};

#[derive(Eq, Hash, PartialEq)]
pub struct Key(u64);

impl From<&Point> for Key {
    fn from(o: &Point) -> Self {
        Key(((o.x as u64) << 32) | (o.y as u64))
    }
}

impl From<Point> for Key {
    fn from(o: Point) -> Self {
        Key(((o.x as u64) << 32) | (o.y as u64))
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

#[derive(Default)]
pub struct DisjointSet(HashMap<usize, usize>);

impl DisjointSet {
    pub fn join(&mut self, this: usize, that: usize) {
        let l = *self.root_mut(this);
        let r = self.root_mut(that);
        *r = l;
    }

    pub fn groups(&self) -> Vec<Vec<usize>> {
        let mut map: HashMap<usize, Vec<usize>> = HashMap::new();
        for (k, v) in self.0.iter() {
            map.entry(self.root(*v)).or_default().push(*k);
        }
        map.into_values().collect()
    }

    fn root(&self, index: usize) -> usize {
        let mut index = index;
        while let Some(upper) = self.0.get(&index).filter(|upper| **upper != index) {
            index = *upper;
        }
        index
    }

    fn root_mut(&mut self, index: usize) -> &mut usize {
        let mut root = index;
        loop {
            let upper = self.parent_mut(root);
            if *upper == root {
                break;
            }
            root = *upper;
        }

        let mut index = index;
        while index != root {
            let upper = self.parent_mut(index);
            index = *upper;
            *upper = root;
        }

        // we have to call it again to avoid non-lexical lifetime issue:
        // https://github.com/rust-lang/rust/issues/21906
        self.parent_mut(root)
    }

    fn parent_mut(&mut self, index: usize) -> &mut usize {
        self.0.entry(index).or_insert(index)
    }
}
