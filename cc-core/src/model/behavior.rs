use super::Movement;

pub struct Behavior(Option<Movement>, Automatic);

impl Behavior {
    pub fn new() -> Self {
        Behavior(None, Automatic::Idle(Idle))
    }

    pub fn from_sequence<'a, I>(is_loop: bool, movements: I) -> Self
    where
        I: Iterator<Item = (Movement, usize)>,
    {
        Behavior(
            None,
            Automatic::Move(Move {
                is_loop,
                movements: movements.collect(),
                count: 0,
                index: 0,
            }),
        )
    }

    pub fn from_others<I>(others: I) -> Self
    where
        I: IntoIterator<Item = Self>,
    {
        let (ms, automatics): (Vec<_>, _) = others.into_iter().map(|x| (x.0, x.1)).unzip();
        let force = if ms.windows(2).all(|w| w[0] == w[1]) {
            ms.first().copied().flatten()
        } else {
            None
        };

        Behavior(force, Automatic::Team(Team(automatics)))
    }

    pub fn get(&self) -> Movement {
        match self.0 {
            Some(m) => m,
            None => self.1.get(),
        }
    }

    pub fn set(&mut self, m: Movement) {
        self.0 = Some(m)
    }

    pub fn done(&self) -> bool {
        self.0.is_none() && self.1.done()
    }

    pub fn next(&mut self) {
        self.0 = None;
        self.1.next();
    }
}

enum Automatic {
    Idle(Idle),
    Move(Move),
    Team(Team),
}

impl Automatic {
    pub fn get(&self) -> Movement {
        match self {
            Automatic::Idle(x) => x.get(),
            Automatic::Move(x) => x.get(),
            Automatic::Team(x) => x.get(),
        }
    }

    pub fn done(&self) -> bool {
        match self {
            Automatic::Idle(x) => x.done(),
            Automatic::Move(x) => x.done(),
            Automatic::Team(x) => x.done(),
        }
    }

    pub fn next(&mut self) {
        match self {
            Automatic::Idle(x) => x.next(),
            Automatic::Move(x) => x.next(),
            Automatic::Team(x) => x.next(),
        }
    }
}

struct Idle;

impl Idle {
    fn get(&self) -> Movement {
        Movement::Idle
    }

    fn done(&self) -> bool {
        true
    }

    fn next(&mut self) {}
}

struct Move {
    // stateless
    is_loop: bool,
    movements: Box<[(Movement, usize)]>,
    // stateful
    count: usize,
    index: usize,
}

impl Move {
    fn get(&self) -> Movement {
        if self.index == self.movements.len() {
            Movement::Idle
        } else {
            self.movements[self.index].0
        }
    }

    fn done(&self) -> bool {
        self.index == self.movements.len()
    }

    fn next(&mut self) {
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

struct Team(Vec<Automatic>);

impl Team {
    fn get(&self) -> Movement {
        match self.0.first().map(|x| x.get()) {
            Some(m) if self.0.iter().skip(1).all(|x| m == x.get()) => m,
            _ => Movement::Idle,
        }
    }

    fn done(&self) -> bool {
        self.0.iter().all(|m| m.done())
    }

    fn next(&mut self) {
        self.0.iter_mut().for_each(|m| m.next());
        self.0.retain(|m| !m.done());
    }
}
