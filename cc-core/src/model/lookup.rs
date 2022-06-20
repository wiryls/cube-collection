use std::collections::{HashMap, HashSet};

use super::{HeadID, Movement, UnitID};
use crate::common::{Adjacence, Neighborhood, Point};

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

pub struct Collision(HashSet<Key>);

impl Collision {
    pub fn new<T, U>(it: T) -> Self
    where
        T: Iterator<Item = U>,
        U: Into<Key>,
    {
        Self(it.map(|k| k.into()).collect())
    }

    pub fn hit<T: Into<Key>>(&self, k: T) -> bool {
        self.0.contains(&k.into())
    }
}

pub struct Faction(HashMap<Key, HeadID>);

impl Faction {
    pub fn new<T, U, V>(it: T) -> Self
    where
        T: Iterator<Item = (HeadID, U)>,
        U: Iterator<Item = V>,
        V: Into<Key>,
    {
        Self(
            it.flat_map(|(k, v)| v.map(move |x| (x.into(), k.clone())))
                .collect(),
        )
    }

    pub fn put<T: Into<Key>>(&mut self, k: T, v: HeadID) {
        self.0.insert(k.into(), v);
    }

    pub fn get<T: Into<Key>>(&self, k: T) -> Option<HeadID> {
        self.0.get(&k.into()).cloned()
    }
}

#[derive(Default)]
pub struct DisjointSet(HashMap<HeadID, HeadID>);

impl DisjointSet {
    pub fn join(&mut self, this: HeadID, that: HeadID) {
        let l = self.root_mut(this).clone();
        let r = self.root_mut(that);
        *r = l;
    }

    pub fn groups(&self) -> impl Iterator<Item = Vec<HeadID>> {
        let mut sets = HashMap::with_capacity(self.0.len() / 2);
        self.0.iter().for_each(|(k, v)| {
            sets.entry(self.root(v.clone()))
                .or_insert_with(Vec::new)
                .push(k.clone())
        });

        sets.into_values()
    }

    fn root(&self, mut index: HeadID) -> HeadID {
        while let Some(upper) = self.0.get(&index).filter(|upper| **upper != index) {
            index = upper.clone();
        }
        index
    }

    fn root_mut(&mut self, mut index: HeadID) -> &mut HeadID {
        let mut root = index.clone();
        loop {
            let upper = self.parent_mut(root.clone());
            if *upper == root {
                break;
            }
            root = upper.clone();
        }

        while index != root {
            let upper = self.parent_mut(index);
            index = upper.clone();
            *upper = root.clone();
        }

        // we have to call it again to avoid non-lexical lifetime issue:
        // https://github.com/rust-lang/rust/issues/21906
        self.parent_mut(root)
    }

    fn parent_mut(&mut self, index: HeadID) -> &mut HeadID {
        self.0.entry(index.clone()).or_insert(index)
    }
}

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
