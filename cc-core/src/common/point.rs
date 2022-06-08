#[derive(Clone, Copy, Default, Eq, PartialEq)]
pub struct Point<T = i32> {
    pub x: T,
    pub y: T,
}
