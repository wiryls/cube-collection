use crate::common::Point;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Movement {
    Idle,
    Left,
    Down,
    Up,
    Right,
}

pub trait Movable {
    fn near(&self, m: Movement) -> Self;
    fn step(&mut self, m: Movement) -> &mut Self;
}

impl Movable for Point {
    fn near(&self, m: Movement) -> Self {
        let mut next = self.clone();
        next.step(m);
        next
    }
    fn step(&mut self, m: Movement) -> &mut Self {
        match m {
            Movement::Idle => (),
            Movement::Left => self.x -= 1,
            Movement::Down => self.y += 1,
            Movement::Up => self.y -= 1,
            Movement::Right => self.x += 1,
        }
        self
    }
}
