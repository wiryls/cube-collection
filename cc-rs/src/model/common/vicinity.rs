#[derive(Clone, Copy, Eq, PartialEq)]
pub struct Adjacence(u8);

impl Adjacence {
    pub const LEFT /*         **/ : Adjacence = Adjacence(0b_10000000);
    pub const LEFT_TOP /*     **/ : Adjacence = Adjacence(0b_01000000);
    pub const TOP /*          **/ : Adjacence = Adjacence(0b_00100000);
    pub const RIGHT_TOP /*    **/ : Adjacence = Adjacence(0b_00010000);
    pub const RIGHT /*        **/ : Adjacence = Adjacence(0b_00001000);
    pub const RIGHT_BOTTOM /* **/ : Adjacence = Adjacence(0b_00000100);
    pub const BOTTOM /*       **/ : Adjacence = Adjacence(0b_00000010);
    pub const LEFT_BOTTOM /*  **/ : Adjacence = Adjacence(0b_00000001);
}

#[derive(Clone, Copy, Default, Eq, PartialEq)]
pub struct Vicinity(pub u8);

impl Vicinity {
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

    pub fn set(&mut self, mask: Adjacence) {
        self.0 |= mask.0;
    }

    pub fn states(&self) -> [bool; 8] {
        Vicinity::AROUND.map(|mask| (self.0 & mask.0) != 0)
    }
}
