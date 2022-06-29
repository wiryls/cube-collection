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
        Self(it.map(Into::into).collect())
    }

    pub fn hit<T: Into<Key>>(&self, k: T) -> bool {
        self.0.contains(&k.into())
    }
}

pub struct Faction(HashMap<Key, HeadID>);

impl Faction {
    pub fn new(it: impl Iterator<Item = (Key, HeadID)>) -> Self {
        Self(it.collect())
    }

    pub fn get<T: Into<Key>>(&self, k: T) -> Option<HeadID> {
        self.0.get(&k.into()).cloned()
    }
}

pub struct DisjointSet {
    parents: Box<[Option<usize>]>,
    existed: Vec<usize>,
}

impl DisjointSet {
    pub fn new(capacity: usize) -> Self {
        Self {
            parents: vec![None; capacity].into(),
            existed: Vec::with_capacity(capacity / 2),
        }
    }

    pub fn join<T: Into<usize>>(&mut self, this: T, that: T) {
        let this = this.into();
        let that = that.into();
        if this < self.parents.len() && that < self.parents.len() {
            *self.root_mut(that) = *self.root_mut(this);
        }
    }

    pub fn groups(self) -> Groups {
        let mut pairs = self
            .existed
            .iter()
            .map(|&i| (i, Self::root(&self.parents, i)))
            .collect::<Box<_>>();
        pairs.sort_by_key(|pair| pair.0);
        Groups(pairs)
    }

    fn root(this: &[Option<usize>], mut index: usize) -> usize {
        loop {
            if let Some(upper) = this[index] {
                if upper != index {
                    index = upper;
                    continue;
                }
            }
            break;
        }
        index
    }

    fn root_mut(&mut self, mut index: usize) -> &mut usize {
        let mut root = index;
        loop {
            let upper = self.parent_mut(root);
            if *upper == root {
                break;
            }
            root = *upper;
        }

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
        self.parents[index].get_or_insert_with(|| {
            self.existed.push(index);
            index
        })
    }
}

pub struct Groups(Box<[(/* self */ usize, /* root */ usize)]>);

impl Groups {
    pub fn iter<'a>(&'a self) -> GroupsIterator<'a> {
        GroupsIterator {
            group: &self,
            index: 0,
        }
    }
}

pub struct GroupsIterator<'a> {
    group: &'a Groups,
    index: usize,
}

impl<'a> Iterator for GroupsIterator<'a> {
    type Item = GroupIterator<'a>;
    fn next(&mut self) -> Option<Self::Item> {
        let lower = self.index;
        let limit = self.group.0.len();
        if limit <= lower {
            return None;
        }

        let mut upper = self.index + 1;
        let value = self.group.0[lower];
        while upper < limit && self.group.0[upper].1 == value.1 {
            upper += 1;
        }

        self.index = upper;
        Some(GroupIterator {
            group: self.group,
            lower,
            upper,
        })
    }
}

pub struct GroupIterator<'a> {
    group: &'a Groups,
    lower: usize,
    upper: usize,
}

impl<'a> Iterator for GroupIterator<'a> {
    type Item = usize;
    fn next(&mut self) -> Option<Self::Item> {
        let index = self.lower;
        if self.upper <= index {
            return None;
        }
        self.lower += 1;
        Some(self.group.0[index].0)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn empty_disjoint_set() {
        let set = DisjointSet::new(0);
        let groups = set.groups();
        assert!(groups.iter().next().is_none());
    }

    #[test]
    fn basic_disjoint_set() {
        let mut set = DisjointSet::new(10);
        set.join(1, 3usize);
        set.join(7, 9usize);
        set.join(5, 7usize);
        set.join(9, 3usize);

        let groups = set.groups();
        let mut it = groups.iter();

        let value = it.next();
        assert!(value.is_some());

        let mut list = Vec::from_iter(value.unwrap());
        list.sort();
        assert_eq!(&list, &[1, 3, 5, 7, 9]);

        let value = it.next();
        assert!(value.is_none());
    }
}
