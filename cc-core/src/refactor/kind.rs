#[allow(dead_code)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum Kind {
    White,
    Red,
    Blue,
    Green,
}

#[allow(dead_code)]
impl Kind {
    pub fn absorbable(self, that: Self) -> bool {
        use Kind::*;
        match self {
            White => false,
            Red => that == Green,
            Blue => that == Red,
            Green => that == Blue,
        }
    }

    pub fn linkable(self, that: Self) -> bool {
        self != Kind::White && self == that
    }
}
