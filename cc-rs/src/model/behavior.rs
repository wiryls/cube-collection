use bevy::utils::tracing::Id;

use super::seed;

#[derive(Clone, Copy, PartialEq)]
pub enum Movement {
    Idle,
    Left,
    Down,
    Up,
    Right,
}

pub trait Behavior {
    fn get(&self) -> Movement;
    fn set(&mut self, m: Movement);
    fn done(&self) -> bool;
    fn next(&mut self);
}

pub fn create_behavior() -> Box<dyn Behavior> {
    Box::new(Idle(None))
}

pub fn create_behavior_from_seed(seed: &seed::Command) -> Box<dyn Behavior> {
    Box::new(Move {
        is_loop: seed.is_loop,
        movements: seed.movements.clone(),
        cache: None,
        count: 0,
        index: 0,
    })
}

pub fn create_behavior_from_behaviors<I>(it: I) -> Box<dyn Behavior>
where
    I: IntoIterator<Item = Box<dyn Behavior>>,
{
    Box::new(Moves {
        cache: None,
        moves: it.into_iter().collect(),
    })
}

struct Done;

impl Behavior for Done {
    fn get(&self) -> Movement {
        Movement::Idle
    }

    fn set(&mut self, _: Movement) {
        // do nothing
    }

    fn done(&self) -> bool {
        true
    }

    fn next(&mut self) {
        // do nothing
    }
}

struct Idle(Option<Movement>);

impl Behavior for Idle {
    fn get(&self) -> Movement {
        self.0.unwrap_or(Movement::Idle)
    }

    fn set(&mut self, m: Movement) {
        self.0 = Some(m)
    }

    fn done(&self) -> bool {
        self.0.is_none()
    }

    fn next(&mut self) {
        self.0 = None
    }
}

struct Move {
    // readonly
    is_loop: bool,
    movements: Vec<(Movement, usize)>,
    // read-write
    cache: Option<Movement>,
    count: usize,
    index: usize,
}

impl Behavior for Move {
    fn get(&self) -> Movement {
        if let Some(m) = self.cache {
            m
        } else if self.index == self.movements.len() {
            Movement::Idle
        } else {
            self.movements[self.index].0
        }
    }

    fn set(&mut self, m: Movement) {
        self.cache = Some(m);
    }

    fn done(&self) -> bool {
        self.cache.is_none() && self.index == self.movements.len()
    }

    fn next(&mut self) {
        self.cache = None;

        let n = self.movements.len();
        if self.index == n {
            return;
        }

        let m = self.movements[self.index].1;
        self.count += 1;
        if self.count == m {
            self.index += 1;
            if self.index == n {
                if self.is_loop {
                    self.index = 0;
                }
            }
            self.count = 0;
        }
    }
}

struct Moves {
    cache: Option<Movement>,
    moves: Vec<Box<dyn Behavior>>,
}

impl Behavior for Moves {
    fn get(&self) -> Movement {
        match self.cache {
            Some(m) => m,
            None => match self.moves.first().map(|x| x.get()) {
                Some(m) if self.moves.iter().skip(1).all(|x| m == x.get()) => m,
                _ => Movement::Idle,
            },
        }
    }

    fn set(&mut self, m: Movement) {
        self.moves.iter_mut().for_each(|it| it.set(m));
    }

    fn done(&self) -> bool {
        !self.moves.iter().any(|m| !m.done())
    }

    fn next(&mut self) {
        self.cache = None;
        self.moves.iter_mut().for_each(|m| m.next());
        self.moves.retain(|m| !m.done());
    }
}
