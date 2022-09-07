use std::{
    borrow::Borrow,
    collections::{HashMap, HashSet},
};

use crate::cube::Point;

/////////////////////////////////////////////////////////////////////////////
// Collision

pub trait Collision {
    fn occupied(&self, point: Point) -> bool;
    fn available(&self, point: Point) -> bool;
    fn put(&mut self, point: Point);
}

pub struct HashSetCollision(HashSet<Point>);

impl HashSetCollision {
    pub fn new<T: Borrow<Point>, I: Iterator<Item = T>>(it: I) -> Self {
        Self(it.map(|x| x.borrow().clone()).collect())
    }
}

impl Collision for HashSetCollision {
    fn occupied(&self, point: Point) -> bool {
        self.0.contains(&point)
    }

    fn available(&self, point: Point) -> bool {
        self.occupied(point)
    }

    fn put(&mut self, point: Point) {
        self.0.insert(point);
    }
}

#[derive(Debug)]
pub struct BitmapCollision {
    width: i32,
    height: i32,
    bits: Box<[u64]>,
}

impl BitmapCollision {
    const UNIT: usize = 64;

    pub fn new(width: usize, height: usize) -> Self {
        let size = (width.max(1) * height.max(1) + Self::UNIT - 1) / Self::UNIT;
        Self {
            width: width as i32,
            height: height as i32,
            bits: vec![0; size].into(),
        }
    }

    fn collapse(&self, point: Point) -> Option<(usize, usize)> {
        if 0 <= point.x && point.x < self.width && 0 <= point.y && point.y < self.height {
            let index = (point.x + point.y * self.width) as usize;
            Some((index / Self::UNIT, index % Self::UNIT))
        } else {
            None
        }
    }
}

impl Collision for BitmapCollision {
    fn occupied(&self, point: Point) -> bool {
        match self.collapse(point) {
            Some((index, delta)) => self.bits[index] & (1 << delta) != 0,
            None => false,
        }
    }

    fn available(&self, point: Point) -> bool {
        match self.collapse(point) {
            Some((index, delta)) => self.bits[index] & (1 << delta) == 0,
            None => false,
        }
    }

    fn put(&mut self, point: Point) {
        if let Some((index, delta)) = self.collapse(point) {
            self.bits[index] |= 1 << delta;
        }
    }
}

/////////////////////////////////////////////////////////////////////////////
// DisjointSet

pub struct DisjointSet {
    parents: Vec<Option<usize>>,
    existed: Vec<usize>,
}

pub type DisjointSetGroups = std::collections::hash_map::IntoValues<usize, Vec<usize>>;

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

    pub fn groups(&mut self) -> DisjointSetGroups {
        let hint = self.existed.len();
        let mut pair = HashMap::with_capacity(hint);
        for &value in self.existed.iter() {
            pair.entry(Self::root(&self.parents, value))
                .or_insert_with(|| Vec::with_capacity(hint))
                .push(value);
        }
        self.parents.clear();
        self.existed.clear();
        pair.into_values()
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

        self.parent_mut(root)
        // We have to call `parent_mut` again to avoid non-lexical lifetime issue:
        // https://github.com/rust-lang/rust/issues/21906
        //
        // Although NLL is enable by default in Rust 1.63, but the following code
        // still not works. It seems we need to wait for the polonius.
        // https://blog.rust-lang.org/2022/08/05/nll-by-default.html
        //
        // loop {
        //     let upper = self.parent_mut(index);
        //     if *upper == root {
        //         return upper;
        //     }
        //     index = *upper;
        //     *upper = root;
        // }
    }

    fn parent_mut(&mut self, index: usize) -> &mut usize {
        self.parents[index].get_or_insert_with(|| {
            self.existed.push(index);
            index
        })
    }
}

/////////////////////////////////////////////////////////////////////////////
// Successors

pub struct Digraph(HashMap<usize, HashSet<usize>>, HashSet<usize>);

pub type DigraphNodeIter<'a> = std::collections::hash_set::Iter<'a, usize>;

impl Digraph {
    pub fn with_capacity(capacity: usize) -> Self {
        Self(HashMap::with_capacity(capacity), HashSet::new())
    }

    pub fn add<F: Into<usize>, T: Into<usize>>(&mut self, from: F, to: T) {
        self.0
            .entry(from.into())
            .or_insert_with(|| HashSet::with_capacity(8))
            .insert(to.into());
    }

    pub fn children<T: Into<usize>>(&self, index: T) -> DigraphNodeIter {
        match self.0.get(&index.into()) {
            Some(set) => set,
            _fallback => &self.1,
        }
        .iter()
    }
}

/////////////////////////////////////////////////////////////////////////////
// Tests

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn collisions() {
        fn case<C: Collision>(mut it: C, tag: &'static str) {
            let put = [(1, 1), (1, 2), (4, 2)].map(Point::from);
            let not = [(-1, -1), (0, 0), (1, 0), (0, 1), (1, 11)].map(Point::from);

            for o in put {
                it.put(o);
            }
            for o in put {
                assert!(it.occupied(o), "{} {:?}", tag, o);
            }
            for o in not {
                assert!(!it.occupied(o), "{} {:?}", tag, o);
            }
        }

        case(BitmapCollision::new(5, 3), "5x3 bitmap");
        case(BitmapCollision::new(10, 10), "10x10 bitmap");
        case(HashSetCollision::new::<Point, _>([].into_iter()), "hashset");
    }

    #[test]
    fn disjoint_set() {
        let cases = [
            (0, vec![], vec![]),
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
                .map(|mut x| {
                    x.sort();
                    x
                })
                .collect::<Vec<_>>();
            out.sort_by_key(|x| x.iter().copied().min());

            assert_eq!(case.2, out, "case {}", i);
        }
    }
}
