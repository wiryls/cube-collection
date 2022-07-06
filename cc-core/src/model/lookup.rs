use std::{
    borrow::Borrow,
    collections::{hash_set, HashMap, HashSet},
};

use super::{HeadID, Movement};
use crate::common::{Neighborhood, Point};

#[derive(Eq, Hash, PartialEq)]
struct Key(u64);

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
    pub fn new(it: impl Iterator<Item = Point>) -> Self {
        Self(it.map(Into::into).collect())
    }

    pub fn hit(&self, point: Point) -> bool {
        self.0.contains(&point.into())
    }

    pub fn neighborhood(&self, point: Point) -> Neighborhood {
        Neighborhood::from(
            Neighborhood::AROUND
                .into_iter()
                .filter(|&o| self.hit(point + o.into())),
        )
    }
}

pub struct Faction(HashMap<Key, HeadID>);

impl Faction {
    pub fn new(it: impl Iterator<Item = (Point, HeadID)>) -> Self {
        Self(it.map(|(key, value)| (key.into(), value)).collect())
    }

    pub fn get(&self, point: Point) -> Option<HeadID> {
        self.0.get(&point.into()).cloned()
    }
}

#[derive(Default)]
pub struct Conflict(HashMap<Key, Race>);

impl Conflict {
    pub fn with_capacity(capacity: usize) -> Self {
        Self(HashMap::with_capacity(capacity))
    }

    pub fn put(&mut self, id: HeadID, movement: Movement, points: impl Iterator<Item = Point>) {
        points.for_each(|point| {
            self.0
                .entry(point.into())
                .or_default()
                .set(movement, id.clone())
        });
    }

    pub fn overlaps(
        &self,
    ) -> impl Iterator<Item = impl Iterator<Item = (HeadID, Movement)> + Clone + '_> + '_ {
        self.0.values().filter(Race::conflict).map(Race::whom)
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

    fn set<T: Into<usize>>(&mut self, movement: Movement, index: T) {
        let i = match movement {
            Movement::Left => 0,
            Movement::Down => 1,
            Movement::Up => 2,
            Movement::Right => 3,
        };
        self.mark |= Race::MASK[i];
        self.data[i] = index.into();
    }

    fn conflict(self: &&Self) -> bool {
        self.mark & self.mark - 1 != 0
    }

    fn whom<T: From<usize>>(&self) -> impl Iterator<Item = (T, Movement)> + Clone + '_ {
        (0..4)
            .into_iter()
            .filter(|i| self.mark & Race::MASK[*i] != 0)
            .map(|i| (self.data[i].into(), Race::MOVE[i]))
    }
}

pub struct DisjointSet {
    parents: Box<[Option<usize>]>,
    existed: Vec<usize>,
}

impl DisjointSet {
    pub fn new(size: usize) -> Self {
        Self {
            parents: vec![None; size].into(),
            existed: Vec::with_capacity(size / 2),
        }
    }

    pub fn join<T: Into<usize>, U: Into<usize>>(&mut self, this: T, that: U) {
        let this = this.into();
        let that = that.into();
        if this < self.parents.len() && that < self.parents.len() && this != that {
            let that = *self.root_mut(that);
            let this = self.root_mut(this);
            if *this != that {
                *this = that;
            }
        }
    }

    pub fn groups(self) -> Groups {
        let mut pairs = self
            .existed
            .iter()
            .map(|&i| (i, Self::root(&self.parents, i)))
            .collect::<Box<_>>();
        pairs.sort_by_key(|pair| pair.1);
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
            group: &self.0,
            index: 0,
        }
    }
}

pub struct GroupsIterator<'a> {
    group: &'a [(usize, usize)],
    index: usize,
}

impl<'a> Iterator for GroupsIterator<'a> {
    type Item = std::iter::Map<std::slice::Iter<'a, (usize, usize)>, fn(&(usize, usize)) -> usize>;
    fn next(&mut self) -> Option<Self::Item> {
        let lower = self.index;
        let limit = self.group.len();
        if limit <= lower {
            return None;
        }

        let mut upper = self.index + 1;
        let value = self.group[lower];
        while upper < limit && self.group[upper].1 == value.1 {
            upper += 1;
        }

        self.index = upper;
        Some(self.group[lower..upper].into_iter().map(|x| x.0))
    }
}

pub struct Successors(Box<[Option<HashSet<HeadID>>]>, HashSet<HeadID>);

impl Successors {
    pub fn new(maximum: usize) -> Self {
        Self(vec![None; maximum].into_boxed_slice(), HashSet::new())
    }

    pub fn insert<T, U>(&mut self, index: T, value: U)
    where
        T: Borrow<HeadID>,
        U: Borrow<HeadID>,
    {
        let index = usize::from(index.borrow());
        let limit = self.0.len();
        if let Some(set) = self.0.get_mut(index) {
            let build = || HashSet::with_capacity(4.max(limit / 4));
            set.get_or_insert_with(build).insert(value.borrow().clone());
        }
    }

    pub fn walk<T: Borrow<HeadID>>(&mut self, index: T) -> hash_set::Iter<'_, HeadID> {
        let index = usize::from(index.borrow());
        match self.0.get(index) {
            Some(Some(set)) => set,
            _ => &self.1,
        }
        .iter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_conflict() {
        let lookup = Conflict::with_capacity(0);
        assert_eq!(lookup.overlaps().count(), 0);
    }

    #[test]
    fn basic_conflict() {
        let cases = [(
            vec![
                (HeadID::from(1), Movement::Right, vec![(1, 0), (1, 1)]),
                (HeadID::from(2), Movement::Up, vec![(1, 0)]),
                (HeadID::from(3), Movement::Left, vec![(1, 0)]),
                (HeadID::from(4), Movement::Left, vec![(1, 1)]),
            ],
            vec![
                vec![
                    (HeadID::from(1), Movement::Right),
                    (HeadID::from(4), Movement::Left),
                ],
                vec![
                    (HeadID::from(1), Movement::Right),
                    (HeadID::from(2), Movement::Up),
                    (HeadID::from(3), Movement::Left),
                ],
            ],
        )];

        for (i, (input, output)) in cases.into_iter().enumerate() {
            let mut lookup = Conflict::with_capacity(input.len());
            for (id, movement, points) in input {
                lookup.put(id, movement, points.into_iter().map(Into::into));
            }

            let expect = HashSet::from_iter(output.iter().cloned());
            let actual = lookup
                .overlaps()
                .map(|x| {
                    let mut v = x.collect::<Vec<_>>();
                    v.sort_by_key(|x| usize::from(&x.0));
                    v
                })
                .collect::<HashSet<_>>();

            assert_eq!(expect, actual, "case {}", i);
        }
    }

    #[test]
    fn empty_disjoint_set() {
        let lookup = DisjointSet::new(0);
        let groups = lookup.groups();
        assert!(groups.iter().next().is_none());
    }

    #[test]
    fn basic_disjoint_set() {
        let cases = [
            (
                10,
                vec![(1, 3), (7, 9), (5, 7), (9usize, 3usize)],
                vec![vec![1, 3, 5, 7, 9usize]],
            ),
            (
                10,
                vec![(6, 1), (3, 1), (9, 1), (0, 2)],
                vec![vec![0, 2], vec![1, 3, 6, 9]],
            ),
        ];

        for (i, case) in cases.into_iter().enumerate() {
            let mut lookup = DisjointSet::new(case.0);
            for link in case.1 {
                lookup.join(link.0, link.1);
            }

            let mut out = lookup
                .groups()
                .iter()
                .map(|x| {
                    let mut v = x.collect::<Vec<_>>();
                    v.sort();
                    v
                })
                .collect::<Vec<_>>();
            out.sort_by_key(|x| x.iter().copied().min());

            assert_eq!(case.2, out, "case {}", i);
        }
    }
}
