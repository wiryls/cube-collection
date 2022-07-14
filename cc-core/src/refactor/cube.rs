use std::rc::Rc;

use super::{
    item::Item,
    kind::Kind,
    lookup::{BitmapCollision, Collision, CollisionExtension, HashSetCollision},
    motion::Motion,
    movement::{Constraint, Movement},
    neighborhood::{Adjacence, Neighborhood},
    point::Point,
};

/////////////////////////////////////////////////////////////////////////////
// export

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct Collection {
    cubes: Vec<Cube>,
    items: Box<[Unit]>,
    fixed: Rc<Area>,
}

#[allow(dead_code)]
impl Collection {
    pub fn link(&mut self) {
        todo!()
    }

    pub fn next(&mut self) {
        todo!()
    }

    pub fn take(&mut self) {
        todo!()
    }

    pub fn r#move(&mut self, input: Option<Movement>) {
        todo!()
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct Cube {
    index: usize,
    kind: Kind,
    units: Vec<Unit>,
    motion: Motion,
    outlines: Outlines,
    movement: Option<Movement>,
    constraint: Constraint,
    successors: Vec<usize>,
}

/////////////////////////////////////////////////////////////////////////////
// internal

#[allow(dead_code)]
#[derive(Clone, Debug)]
struct Unit {
    index: usize,
    position: Point,
    neighborhood: Neighborhood,
}

#[derive(Debug)]
pub struct Area {
    cubes: Box<[(Point, Neighborhood)]>,
    impassable: BitmapCollision,
}

#[allow(dead_code)]
impl Area {
    pub fn new<'a, I>(width: usize, height: usize, it: I) -> Self
    where
        I: Iterator<Item = &'a [Point]>,
    {
        let build = |os: &'a [Point]| {
            let c = HashSetCollision::new(os.iter());
            os.iter().map(move |&o| (o, c.neighborhood(o)))
        };

        let cubes = it.flat_map(build).collect::<Box<_>>();
        let mut impassable = BitmapCollision::new(width, height);
        cubes.iter().for_each(|x| impassable.put(x.0));

        Self { cubes, impassable }
    }

    pub fn blocked(&self, point: Point) -> bool {
        self.impassable.hit(point)
    }

    pub fn iter(&self, offset: usize) -> AreaIter {
        AreaIter {
            offset,
            iterator: self.cubes.iter().enumerate(),
        }
    }
}

pub struct AreaIter<'a> {
    offset: usize,
    iterator: std::iter::Enumerate<std::slice::Iter<'a, (Point, Neighborhood)>>,
}

impl Iterator for AreaIter<'_> {
    type Item = Item;

    fn next(&mut self) -> Option<Self::Item> {
        self.iterator
            .next()
            .map(|(index, (point, neighborhood))| Item {
                id: (index + self.offset).into(),
                kind: Kind::White,
                position: point.clone(),
                movement: None,
                constraint: Constraint::Free,
                neighborhood: neighborhood.clone(),
            })
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
struct Outlines {
    count: [usize; 3],
    slice: Box<[Point]>,
}

#[allow(dead_code)]
impl Outlines {
    pub fn new(units: &[Unit]) -> Self {
        const LBTR: [Adjacence; 4] = [
            Adjacence::LEFT,
            Adjacence::BOTTOM,
            Adjacence::TOP,
            Adjacence::RIGHT,
        ];

        // count:
        // [  0     1       2    3   ]
        // 0..left..bottom..top..right
        let mut count: [usize; 4] = Default::default();
        for unit in units.iter() {
            for (i, a) in LBTR.into_iter().enumerate() {
                if !unit.neighborhood.has(a) {
                    count[i] += 1;
                }
            }
        }
        for i in 1..count.len() {
            count[i] += count[i - 1];
        }

        let mut slice = {
            let first = Self::anchor(units);
            let total = count[3];
            vec![first; total]
        };
        let mut index = {
            let mut index = count.clone();
            index.rotate_right(1);
            index[0] = 0;
            index
        };
        for unit in units.iter() {
            for (i, a) in LBTR.into_iter().enumerate() {
                if !unit.neighborhood.has(a) {
                    let point = &mut slice[index[i]];
                    *point -= unit.position;
                    *point -= a.into();
                    index[i] += 1;
                }
            }
        }

        Self {
            count: [count[0], count[1], count[2]],
            slice: slice.into(),
        }
    }

    pub fn one(&self, anchor: Point, m: Movement) -> impl Iterator<Item = Point> + Clone + '_ {
        use Movement::*;
        match m {
            Left /***/ => &self.slice[..self.count[0]],
            Down /***/ => &self.slice[self.count[0]..self.count[1]],
            Up /*****/ => &self.slice[self.count[1]..self.count[2]],
            Right /**/ => &self.slice[self.count[2]..],
        }
        .iter()
        .map(move |o| anchor - *o)
    }

    pub fn all(&self, anchor: Point) -> impl Iterator<Item = Point> + Clone + '_ {
        (&self.slice[..]).iter().map(move |o| anchor - *o)
    }

    fn anchor(units: &[Unit]) -> Point {
        units.first().map(|u| u.position).unwrap_or_default()
    }
}

/////////////////////////////////////////////////////////////////////////////
// tests

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn outlines() {
        let units = vec![
            Unit {
                index: 0,
                position: Point::new(0, 0),
                neighborhood: Neighborhood::from([Adjacence::BOTTOM].into_iter()),
            },
            Unit {
                index: 0,
                position: Point::new(0, 1),
                neighborhood: Neighborhood::from([Adjacence::TOP].into_iter()),
            },
        ];
        let outlines = Outlines::new(&units);

        let expected = vec![Point::new(0, 1), Point::new(0, 2)];
        let actual = Vec::from_iter(outlines.one(Point::new(1, 1), Movement::Left));
        assert_eq!(actual, expected);

        let expected = vec![Point::new(2, 1), Point::new(2, 2)];
        let actual = Vec::from_iter(outlines.one(Point::new(1, 1), Movement::Right));
        assert_eq!(actual, expected);

        let expected = vec![Point::new(1, 0)];
        let actual = Vec::from_iter(outlines.one(Point::new(1, 1), Movement::Up));
        assert_eq!(actual, expected);

        let expected = vec![Point::new(1, 3)];
        let actual = Vec::from_iter(outlines.one(Point::new(1, 1), Movement::Down));
        assert_eq!(actual, expected);
    }
}
