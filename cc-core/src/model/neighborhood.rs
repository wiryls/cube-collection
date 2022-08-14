use super::point::Point;

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

impl Into<Point> for Adjacence {
    fn into(self) -> Point {
        const POINT_LEFT /*         **/: Point = Point::new(-1, 0);
        const POINT_LEFT_TOP /*     **/: Point = Point::new(-1, -1);
        const POINT_TOP /*          **/: Point = Point::new(0, -1);
        const POINT_RIGHT_TOP /*    **/: Point = Point::new(1, -1);
        const POINT_RIGHT /*        **/: Point = Point::new(1, 0);
        const POINT_RIGHT_BOTTOM /* **/: Point = Point::new(1, 1);
        const POINT_BOTTOM /*       **/: Point = Point::new(0, 1);
        const POINT_LEFT_BOTTOM /*  **/: Point = Point::new(-1, 1);
        const POINT_NONE /*         **/: Point = Point::new(0, 0);

        match self {
            Adjacence::LEFT => POINT_LEFT,
            Adjacence::LEFT_TOP => POINT_LEFT_TOP,
            Adjacence::TOP => POINT_TOP,
            Adjacence::RIGHT_TOP => POINT_RIGHT_TOP,
            Adjacence::RIGHT => POINT_RIGHT,
            Adjacence::RIGHT_BOTTOM => POINT_RIGHT_BOTTOM,
            Adjacence::BOTTOM => POINT_BOTTOM,
            Adjacence::LEFT_BOTTOM => POINT_LEFT_BOTTOM,
            _ => POINT_NONE,
        }
    }
}

impl Into<Point> for &Adjacence {
    fn into(self) -> Point {
        self.to_owned().into()
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Neighborhood(u8);

impl Neighborhood {
    pub const AROUNDS: [Adjacence; 8] = [
        Adjacence::LEFT,
        Adjacence::LEFT_TOP,
        Adjacence::TOP,
        Adjacence::RIGHT_TOP,
        Adjacence::RIGHT,
        Adjacence::RIGHT_BOTTOM,
        Adjacence::BOTTOM,
        Adjacence::LEFT_BOTTOM,
    ];

    pub const CROSS: Neighborhood = Neighborhood(
        Adjacence::LEFT.0 | Adjacence::TOP.0 | Adjacence::RIGHT.0 | Adjacence::BOTTOM.0,
    );

    pub const fn new() -> Self {
        Neighborhood(0)
    }

    pub fn from(it: impl Iterator<Item = Adjacence>) -> Self {
        let mut mask = Self::new();
        it.for_each(|a| mask.set(a));
        mask
    }

    pub const fn has(&self, mask: Adjacence) -> bool {
        self.0 & mask.0 != 0
    }

    pub const fn contains(&self, other: &Self) -> bool {
        self.0 & other.0 == other.0
    }

    pub fn states(&self) -> [bool; 8] {
        Neighborhood::AROUNDS.map(|mask| (self.0 & mask.0) != 0)
    }

    pub fn set(&mut self, mask: Adjacence) {
        self.0 |= mask.0;
    }

    pub fn unset(&mut self, mask: Adjacence) {
        self.0 &= !mask.0;
    }
}
