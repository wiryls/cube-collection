#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum Kind {
    White,
    Green,
    Blue,
    Red,
}

impl Kind {
    pub const fn absorbable(self, other: Self) -> bool {
        use Kind::*;
        match self {
            White => false,
            Green => matches!(other, Blue),
            Blue => matches!(other, Red),
            Red => matches!(other, Green),
        }
    }

    pub const fn linkable(self, other: Self) -> bool {
        use Kind::*;
        match self {
            White => false,
            Green => matches!(other, Green),
            Blue => matches!(other, Blue),
            Red => matches!(other, Red),
        }
    }
}
