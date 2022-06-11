#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Type {
    White,
    Red,
    Blue,
    Green,
}

impl Type {
    pub fn unstable(&self) -> bool {
        *self != Type::White
    }

    pub fn absorbable(&self, that: &Self) -> bool {
        use Type::*;
        match self {
            White => false,
            Red => matches!(that, Red | Blue | Green),
            Blue => matches!(that, Blue | Green),
            Green => matches!(that, Green),
        }
    }

    pub fn absorbable_actively(&self, that: &Self) -> bool {
        self.absorbable(that) && !that.absorbable(self)
    }
}
