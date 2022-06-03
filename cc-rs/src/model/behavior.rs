use super::seed;

#[derive(Clone, Copy, PartialEq)]
pub enum Movement {
    Idle,
    Left,
    Down,
    Up,
    Right,
}

pub struct Behavior(Behaviour);

#[allow(dead_code)]
impl Behavior {
    pub fn new() -> Self {
        Behavior(Behaviour::Idle(Idle(None)))
    }

    pub fn new_with_seed(seed: &seed::Command) -> Self {
        Behavior(Behaviour::Move(Move {
            is_loop: seed.is_loop,
            movements: seed.movements.clone(),
            cache: None,
            count: 0,
            index: 0,
        }))
    }

    #[allow(dead_code)]
    pub fn new_with_others<'a, I>(it: I) -> Self
    where
        I: IntoIterator<Item = Self>,
    {
        Behavior(Behaviour::Team(Team {
            cache: None,
            moves: it.into_iter().map(|x| x.0).filter(|x| !x.done()).collect(),
        }))
    }

    pub fn get(&self) -> Movement {
        self.0.get()
    }

    pub fn set(&mut self, m: Movement) {
        self.0.set(m)
    }

    pub fn done(&self) -> bool {
        self.0.done()
    }

    pub fn next(&mut self) {
        self.0.next()
    }
}

enum Behaviour {
    Idle(Idle),
    Move(Move),
    Team(Team),
}

impl Behaviour {
    pub fn get(&self) -> Movement {
        match self {
            Behaviour::Idle(x) => x.get(),
            Behaviour::Move(x) => x.get(),
            Behaviour::Team(x) => x.get(),
        }
    }

    pub fn set(&mut self, m: Movement) {
        match self {
            Behaviour::Idle(x) => x.set(m),
            Behaviour::Move(x) => x.set(m),
            Behaviour::Team(x) => x.set(m),
        }
    }

    pub fn done(&self) -> bool {
        match self {
            Behaviour::Idle(x) => x.done(),
            Behaviour::Move(x) => x.done(),
            Behaviour::Team(x) => x.done(),
        }
    }

    pub fn next(&mut self) {
        match self {
            Behaviour::Idle(x) => x.next(),
            Behaviour::Move(x) => x.next(),
            Behaviour::Team(x) => x.next(),
        }
    }
}

struct Idle(Option<Movement>);

impl Idle {
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

impl Move {
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

struct Team {
    cache: Option<Movement>,
    moves: Vec<Behaviour>,
}

impl Team {
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
        self.cache = Some(m);
        self.moves.iter_mut().for_each(|it| it.set(m));
    }

    fn done(&self) -> bool {
        self.moves.iter().all(|m| m.done())
    }

    fn next(&mut self) {
        self.cache = None;
        self.moves.iter_mut().for_each(|m| m.next());
        self.moves.retain(|m| !m.done());
    }
}
