/// The type of cubes.
#[derive(Clone, Copy, PartialEq)]
pub enum Type {
    White,
    Red,
    Blue,
    Green,
}

#[allow(dead_code)]
impl Type {
    pub fn is_active(&self) -> bool {
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
}
