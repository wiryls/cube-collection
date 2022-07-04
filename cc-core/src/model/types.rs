#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Kind {
    White,
    Red,
    Blue,
    Green,
}

impl Kind {
    pub fn unstable(&self) -> bool {
        *self != Kind::White
    }

    pub fn absorbable(&self, that: Self) -> bool {
        use Kind::*;
        match self {
            White => false,
            Red => matches!(that, Red | Blue | Green),
            Blue => matches!(that, Blue | Green),
            Green => matches!(that, Green),
        }
    }

    pub fn absorbable_actively(&self, that: Self) -> bool {
        self.absorbable(that) && !that.absorbable(self.clone())
    }
}
