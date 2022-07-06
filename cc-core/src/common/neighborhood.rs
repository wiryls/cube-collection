use super::Point;

#[derive(Clone, Copy, Default, Eq, PartialEq)]
pub struct Adjacence(u8);

impl Adjacence {
    pub const LEFT /*         **/: Adjacence = Adjacence(0b_10000000);
    pub const LEFT_TOP /*     **/: Adjacence = Adjacence(0b_01000000);
    pub const TOP /*          **/: Adjacence = Adjacence(0b_00100000);
    pub const RIGHT_TOP /*    **/: Adjacence = Adjacence(0b_00010000);
    pub const RIGHT /*        **/: Adjacence = Adjacence(0b_00001000);
    pub const RIGHT_BOTTOM /* **/: Adjacence = Adjacence(0b_00000100);
    pub const BOTTOM /*       **/: Adjacence = Adjacence(0b_00000010);
    pub const LEFT_BOTTOM /*  **/: Adjacence = Adjacence(0b_00000001);
}

impl Adjacence {
    const POINT_LEFT /*         **/: Point = Point::new(-1, 0);
    const POINT_LEFT_TOP /*     **/: Point = Point::new(-1, -1);
    const POINT_TOP /*          **/: Point = Point::new(0, -1);
    const POINT_RIGHT_TOP /*    **/: Point = Point::new(1, -1);
    const POINT_RIGHT /*        **/: Point = Point::new(1, 0);
    const POINT_RIGHT_BOTTOM /* **/: Point = Point::new(1, 1);
    const POINT_BOTTOM /*       **/: Point = Point::new(0, 1);
    const POINT_LEFT_BOTTOM /*  **/: Point = Point::new(-1, 1);
    const POINT_NONE /*         **/: Point = Point::new(0, 0);
}

impl Into<Point> for Adjacence {
    fn into(self) -> Point {
        match self {
            Adjacence::LEFT => Adjacence::POINT_LEFT,
            Adjacence::LEFT_TOP => Adjacence::POINT_LEFT_TOP,
            Adjacence::TOP => Adjacence::POINT_TOP,
            Adjacence::RIGHT_TOP => Adjacence::POINT_RIGHT_TOP,
            Adjacence::RIGHT => Adjacence::POINT_RIGHT,
            Adjacence::RIGHT_BOTTOM => Adjacence::POINT_RIGHT_BOTTOM,
            Adjacence::BOTTOM => Adjacence::POINT_BOTTOM,
            Adjacence::LEFT_BOTTOM => Adjacence::POINT_LEFT_BOTTOM,
            _ => Adjacence::POINT_NONE,
        }
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct Neighborhood(u8);

#[allow(dead_code)]
impl Neighborhood {
    pub const AROUND: [Adjacence; 8] = [
        Adjacence::LEFT,
        Adjacence::LEFT_TOP,
        Adjacence::TOP,
        Adjacence::RIGHT_TOP,
        Adjacence::RIGHT,
        Adjacence::RIGHT_BOTTOM,
        Adjacence::BOTTOM,
        Adjacence::LEFT_BOTTOM,
    ];

    pub fn new() -> Self {
        Self::default()
    }

    pub fn from(it: impl Iterator<Item = Adjacence>) -> Self {
        let mut n = Self::new();
        it.for_each(|a| n.set(a));
        n
    }

    pub fn set(&mut self, mask: Adjacence) {
        self.0 |= mask.0;
    }

    pub fn unset(&mut self, mask: Adjacence) {
        self.0 &= !mask.0;
    }

    pub fn has(&self, mask: Adjacence) -> bool {
        self.0 & mask.0 != 0
    }

    pub fn states(&self) -> [bool; 8] {
        Neighborhood::AROUND.map(|mask| (self.0 & mask.0) != 0)
    }
}
