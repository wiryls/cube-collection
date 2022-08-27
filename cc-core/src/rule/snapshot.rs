use std::sync::Arc;

use super::{
    frozen::Frozen,
    item::{Diff, Item},
};
use crate::cube::{Constraint, Kind, Neighborhood, Point};

#[derive(Clone, Debug)]
pub struct Snapshot {
    active: Vec<Item>,
    forzen: Arc<Frozen>,
}

impl Snapshot {
    pub(crate) fn new(active: Vec<Item>, forzen: Arc<Frozen>) -> Self {
        Self { active, forzen }
    }

    pub fn contains(&self, position: Point) -> bool {
        self.active.iter().any(|unit| unit.position == position) || self.forzen.blocked(position)
    }

    pub fn differ<'a>(&'a self, that: &'a Self) -> impl Iterator<Item = Diff> + 'a {
        use std::ptr::eq;
        let same = eq(self, that);
        let same_source = eq(self.forzen.as_ref(), that.forzen.as_ref());
        let comparable = same_source && self.active.len() == that.active.len();
        let maximum = (!same && comparable) as usize * self.active.len();

        std::iter::zip(self.active.iter(), that.active.iter())
            .take(maximum)
            .filter(|(l, r)| {
                l.kind != r.kind
                    || l.position != r.position
                    || l.movement != r.movement
                    || l.constraint != r.constraint
                    || l.neighborhood != r.neighborhood
            })
            .map(|(l, r)| Diff {
                id: r.id,
                kind: (l.kind != r.kind).then(|| r.kind),
                position: (l.position != r.position).then(|| r.position),
                movement: (l.movement != r.movement).then(|| r.movement),
                constraint: (l.constraint != r.constraint).then(|| r.constraint),
                neighborhood: (l.neighborhood != r.neighborhood).then(|| r.neighborhood),
            })
    }

    pub fn iter(&self) -> SnapshotIter<'_> {
        SnapshotIter {
            source: self,
            primary: Some(self.active.iter()),
            secondary: Some(self.forzen.iter().enumerate()),
        }
    }
}

pub struct SnapshotIter<'a> {
    source: &'a Snapshot,
    primary: Option<std::slice::Iter<'a, Item>>,
    secondary: Option<std::iter::Enumerate<std::slice::Iter<'a, (Point, Neighborhood)>>>,
}

impl<'a> Iterator for SnapshotIter<'a> {
    type Item = Item;

    fn size_hint(&self) -> (usize, Option<usize>) {
        let x = self.source.active.len() + self.source.forzen.len();
        (x, Some(x))
    }

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(iter) = &mut self.primary {
            if let Some(output) = iter.next() {
                return Some(output.clone());
            }
            self.primary = None
        }

        if let Some(iter) = &mut self.secondary {
            if let Some((index, (point, neighborhood))) = iter.next() {
                return Some(Item {
                    id: index + self.source.active.len(),
                    kind: Kind::White,
                    position: point.clone(),
                    movement: None,
                    constraint: Constraint::Free,
                    neighborhood: neighborhood.clone(),
                });
            }
            self.secondary = None
        }

        None
    }
}
