use super::Movement;

#[derive(Clone)]
pub struct Motion(Automatic);

impl Motion {
    pub fn new() -> Self {
        Motion(Automatic::Idle)
    }

    pub fn from_sequence<'a, I>(is_loop: bool, movements: I) -> Self
    where
        I: Iterator<Item = (Movement, usize)>,
    {
        Motion(Automatic::Move(Move {
            is_loop,
            movements: movements.collect(),
            count: 0,
            index: 0,
        }))
    }

    pub fn from_iter<'a, I>(others: I) -> Self
    where
        I: Iterator<Item = &'a Self>,
    {
        let mut auto = others.map(|x| &x.0).collect::<Vec<_>>();
        auto.retain(|x| !matches!(x, Automatic::Idle));
        match auto.len() {
            0 => Motion(Automatic::Idle),
            1 => Motion(auto.into_iter().next().cloned().unwrap_or_default()),
            _ => Motion(Automatic::Team(Team(auto.into_iter().cloned().collect()))),
        }
    }

    pub fn get(&self) -> Movement {
        self.0.get()
    }

    pub fn done(&self) -> bool {
        self.0.done()
    }

    pub fn next(&mut self) {
        self.0.next();
    }

    pub fn take(&mut self) -> Self {
        Self(std::mem::take(&mut self.0))
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
