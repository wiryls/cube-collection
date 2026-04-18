use std::sync::Arc;

use super::{
    frozen::Frozen,
    output::{Diff, Unit},
};
use crate::cube::{Constraint, Kind, Neighborhood, Point};

#[derive(Clone, Debug)]
pub struct Snapshot {
    active: Vec<Unit>,
    frozen: Arc<Frozen>,
}

impl Snapshot {
    pub(crate) fn new(active: Vec<Unit>, frozen: Arc<Frozen>) -> Self {
        Self { active, frozen }
    }

    pub fn contains(&self, position: Point) -> bool {
        self.active.iter().any(|unit| unit.position == position) || self.frozen.blocked(position)
    }

    pub fn differ<'a>(&'a self, that: &'a Self) -> impl Iterator<Item = Diff> + 'a {
        use std::ptr::eq;
        let same = eq(self, that);
        let same_source = eq(self.frozen.as_ref(), that.frozen.as_ref());
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
                kind: (l.kind != r.kind).then_some(r.kind),
                position: (l.position != r.position).then_some(r.position),
                movement: (l.movement != r.movement).then_some(r.movement),
                constraint: (l.constraint != r.constraint).then_some(r.constraint),
                neighborhood: (l.neighborhood != r.neighborhood).then_some(r.neighborhood),
            })
    }

    pub fn iter(&self) -> SnapshotIter<'_> {
        SnapshotIter {
            source: self,
            primary: Some(self.active.iter()),
            secondary: Some(self.frozen.iter().enumerate()),
        }
    }
}

pub struct SnapshotIter<'a> {
    source: &'a Snapshot,
    primary: Option<std::slice::Iter<'a, Unit>>,
    secondary: Option<std::iter::Enumerate<std::slice::Iter<'a, (Point, Neighborhood)>>>,
}

impl<'a> Iterator for SnapshotIter<'a> {
    type Item = Unit;

    fn size_hint(&self) -> (usize, Option<usize>) {
        let x = self.source.active.len() + self.source.frozen.len();
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
                return Some(Unit {
                    id: index + self.source.active.len(),
                    kind: Kind::White,
                    position: *point,
                    movement: None,
                    constraint: Constraint::Free,
                    neighborhood: *neighborhood,
                });
            }
            self.secondary = None
        }

        None
    }
}
