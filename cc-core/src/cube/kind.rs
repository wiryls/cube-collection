#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum Kind {
    White,
    Red,
    Blue,
    Green,
}

impl Kind {
    pub const fn absorbable(self, that: Self) -> bool {
        use Kind::*;
        match self {
            White => false,
            Red => matches!(that, Green),
            Blue => matches!(that, Red),
            Green => matches!(that, Blue),
        }
    }

    pub const fn linkable(self, that: Self) -> bool {
        use Kind::*;
        match self {
            White => false,
            Red => matches!(that, Red),
            Blue => matches!(that, Blue),
            Green => matches!(that, Green),
        }
    }
}
