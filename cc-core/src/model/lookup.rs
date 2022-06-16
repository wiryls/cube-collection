use std::collections::{HashMap, HashSet};

use super::HeadID;
use crate::common::Point;

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

    pub fn groups(&self) -> Vec<Vec<HeadID>> {
        let mut map = HashMap::with_capacity(self.0.len() / 2);
        self.0.iter().for_each(|(k, v)| {
            map.entry(self.root(v.clone()))
                .or_insert_with(Vec::new)
                .push(k.clone())
        });

        map.into_values()
            .map(|mut x| {
                x.sort();
                x
            })
            .collect()
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
