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
