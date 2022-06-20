use super::Movement;

#[derive(Clone)]
pub struct Motion(Option<Movement>, Automatic);

impl Motion {
    pub fn new() -> Self {
        Motion(None, Automatic::Idle)
    }

    pub fn from_sequence<'a, I>(is_loop: bool, movements: I) -> Self
    where
        I: Iterator<Item = (Movement, usize)>,
    {
        Motion(
            None,
            Automatic::Move(Move {
                is_loop,
                movements: movements.collect(),
                count: 0,
                index: 0,
            }),
        )
    }

    pub fn from_iter<I>(others: I) -> Self
    where
        I: IntoIterator<Item = Self>,
    {
        let (m, mut automatics): (Vec<_>, Vec<_>) = others.into_iter().map(|x| (x.0, x.1)).unzip();

        let m = if m.windows(2).all(|w| w[0] == w[1]) {
            m.first().copied().flatten()
        } else {
            None
        };

        automatics.retain(|a| !matches!(a, Automatic::Idle));
        match automatics.len() {
            0 => Motion(m, Automatic::Idle),
            1 => Motion(m, automatics.into_iter().next().unwrap_or_default()),
            _ => Motion(m, Automatic::Team(Team(automatics))),
        }
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

    pub fn take(&mut self) -> Self {
        Self(self.0.take(), std::mem::take(&mut self.1))
    }
}

#[derive(Clone)]
enum Automatic {
    Idle,
    Move(Move),
    Team(Team),
}

impl Automatic {
    pub fn get(&self) -> Movement {
        match self {
            Self::Idle => Movement::Idle,
            Self::Move(x) => x.get(),
            Self::Team(x) => x.get(),
        }
    }

    pub fn done(&self) -> bool {
        match self {
            Self::Idle => true,
            Self::Move(x) => x.done(),
            Self::Team(x) => x.done(),
        }
    }

    pub fn next(&mut self) {
        match self {
            Self::Idle => return,
            Self::Move(x) => x.next(),
            Self::Team(x) => x.next(),
        }
        if self.done() {
            *self = Self::Idle;
        }
    }
}

impl Default for Automatic {
    fn default() -> Self {
        Self::Idle
    }
}

#[derive(Clone)]
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

#[derive(Clone)]
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
