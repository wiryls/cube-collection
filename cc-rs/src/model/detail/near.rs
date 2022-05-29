#[derive(Clone, Copy, Eq, PartialEq)]
pub struct Direction(u8);

impl Direction {
    pub const LEFT /*         **/ : Direction = Direction(0b_1000_0000);
    pub const LEFT_TOP /*     **/ : Direction = Direction(0b_0100_0000);
    pub const TOP /*          **/ : Direction = Direction(0b_0010_0000);
    pub const RIGHT_TOP /*    **/ : Direction = Direction(0b_0001_0000);
    pub const RIGHT /*        **/ : Direction = Direction(0b_0000_1000);
    pub const RIGHT_BOTTOM /* **/ : Direction = Direction(0b_0000_0100);
    pub const BOTTOM /*       **/ : Direction = Direction(0b_0000_0010);
    pub const LEFT_BOTTOM /*  **/ : Direction = Direction(0b_0000_0001);
}

#[derive(Clone, Copy, Default, Eq, PartialEq)]
pub struct Near(pub u8);

impl Near {
    pub const AROUND: [Direction; 8] = [
        Direction::LEFT,
        Direction::LEFT_TOP,
        Direction::TOP,
        Direction::RIGHT_TOP,
        Direction::RIGHT,
        Direction::RIGHT_BOTTOM,
        Direction::BOTTOM,
        Direction::LEFT_BOTTOM,
    ];

    pub fn new() -> Self {
        Self::default()
    }

    pub fn set(&mut self, bit: Direction) {
        self.0 |= bit.0;
    }

    pub fn around(&self) -> [bool; 8] {
        Near::AROUND.map(|mask| (self.0 & mask.0) != 0)
    }
}
