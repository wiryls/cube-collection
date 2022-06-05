use super::behavior::Movement;
use crate::model::common::*;
use bevy::math::Rect;

pub struct Unibody {
    // necessary
    rect: Rect<i32>,
    units: Vec<Point>,
    // cached for queries
    tests: Existence,
    edges: Borders,
}

#[allow(dead_code)]
impl Unibody {
    pub fn new<'a, I, U, V>(it: I) -> Self
    where
        I: Iterator<Item = &'a U>,
        U: 'a + Location<V>,
        V: Into<i32>,
    {
        // [0] collect points into units
        let mut units: Vec<Point> = it.map(Point::from).collect();

        // [1] create rect
        let rect = surrounding(&units);

        // [2] relocate units
        for unit in units.iter_mut() {
            unit.x -= rect.left;
            unit.y -= rect.top;
        }

        // [3] create tests table
        let tests = Existence::new(units.iter());

        // [4] create edges table
        let edges = Borders::new(&units, &tests);

        // [3] finish
        Self {
            rect,
            units,
            tests,
            edges,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.units.is_empty()
    }

    pub fn units(&self) -> impl Iterator<Item = Point> + '_ {
        self.units.iter().map(|x| absolute(&self.rect, x))
    }

    pub fn edges(&self, m: Movement) -> impl Iterator<Item = Point> + '_ {
        self.edges.get(m).iter().map(|x| absolute(&self.rect, x))
    }

    pub fn calculate_patterns(&self) -> impl Iterator<Item = (&Point, CubePattern)> {
        self.units
            .iter()
            .map(|unit| {
                let mut pattern = CubePattern::new();
                for direction in CubePattern::AROUND {
                    if self.tests.has(&unit.near(direction)) {
                        pattern.set(direction);
                    }
                }
                (unit, pattern)
            })
            .into_iter()
    }

    // TODO: implement merge methods.
    // pub fn merge(&mut self, that: Self) {}
}

impl Location<i32> for Unibody {
    fn x(&self) -> i32 {
        self.rect.left
    }
    fn y(&self) -> i32 {
        self.rect.top
    }
}

#[derive(Default)]
struct Borders {
    size: [usize; 3],
    data: Vec<Point>,
}

impl Borders {
    fn new(os: &Vec<Point>, ex: &Existence) -> Self {
        let mut size: [usize; 3] = [0, 0, 0];
        let mut data: Vec<Point> = Vec::new();

        use Movement::*;
        data.extend(os.iter().map(|o| o.step(Left)).filter(|o| !ex.has(o)));
        size[0] = data.len();
        data.extend(os.iter().map(|o| o.step(Down)).filter(|o| !ex.has(o)));
        size[1] = data.len();
        data.extend(os.iter().map(|o| o.step(Up)).filter(|o| !ex.has(o)));
        size[2] = data.len();
        data.extend(os.iter().map(|o| o.step(Right)).filter(|o| !ex.has(o)));

        Self { size, data }
    }

    fn get(&self, m: Movement) -> &[Point] {
        match m {
            Movement::Idle => self.data.as_slice(),
            Movement::Left => &self.data[0..self.size[0]],
            Movement::Down => &self.data[self.size[0]..self.size[1]],
            Movement::Up => &self.data[self.size[1]..self.size[2]],
            Movement::Right => &self.data[self.size[2]..],
        }
    }
}

fn absolute(r: &Rect<i32>, o: &Point) -> Point {
    Point {
        x: o.x + r.left,
        y: o.y + r.top,
    }
}

fn surrounding(os: &Vec<Point>) -> Rect<i32> {
    Rect {
        left: os.iter().map(|o| o.x).min().unwrap_or_default(),
        right: os.iter().map(|o| o.x).max().unwrap_or_default(),
        top: os.iter().map(|o| o.y).min().unwrap_or_default(),
        bottom: os.iter().map(|o| o.y).max().unwrap_or_default(),
    }
}
