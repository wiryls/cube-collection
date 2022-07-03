use super::Movement;

#[derive(Clone)]
pub struct Motion(Any);

impl Motion {
    pub fn new() -> Self {
        Motion(Any::Stop)
    }

    pub fn from_sequence<'a, I>(is_loop: bool, movements: I) -> Self
    where
        I: Iterator<Item = (Option<Movement>, usize)>,
    {
        Motion(Any::Move(Move {
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
        auto.retain(|x| !matches!(x, Any::Stop));
        match auto.len() {
            0 => Motion(Any::Stop),
            1 => Motion(auto.into_iter().next().cloned().unwrap_or_default()),
            _ => Motion(Any::Team(Buffered::new(Team(
                auto.into_iter().cloned().collect(),
            )))),
        }
    }

    pub fn current(&self) -> Option<Movement> {
        self.0.current()
    }

    pub fn next(&mut self) {
        self.0.next();
    }

    pub fn take(&mut self) -> Self {
        Self(std::mem::take(&mut self.0))
    }
}

trait Automatic {
    fn current(&self) -> Option<Movement>;
    fn running(&self) -> bool;
    fn next(&mut self);
}

#[derive(Clone)]
enum Any {
    Stop,
    Move(Move),
    Team(Buffered<Team>),
}

impl Default for Any {
    fn default() -> Self {
        Self::Stop
    }
}

impl Automatic for Any {
    fn current(&self) -> Option<Movement> {
        match self {
            Self::Stop => None,
            Self::Move(x) => x.current(),
            Self::Team(x) => x.current(),
        }
    }

    fn running(&self) -> bool {
        match self {
            Self::Stop => false,
            Self::Move(x) => x.running(),
            Self::Team(x) => x.running(),
        }
    }

    fn next(&mut self) {
        match self {
            Self::Stop => return,
            Self::Move(x) => x.next(),
            Self::Team(x) => x.next(),
        }
        if self.running() {
            *self = Self::Stop;
        }
    }
}

#[derive(Clone)]
struct Move {
    // stateless
    is_loop: bool,
    movements: Box<[(Option<Movement>, usize)]>,
    // stateful
    count: usize,
    index: usize,
}

impl Automatic for Move {
    fn current(&self) -> Option<Movement> {
        if self.index == self.movements.len() {
            None
        } else {
            self.movements[self.index].0
        }
    }

    fn running(&self) -> bool {
        self.index != self.movements.len()
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
struct Team(Vec<Any>);

impl Automatic for Team {
    fn current(&self) -> Option<Movement> {
        match self.0.first().map(Automatic::current) {
            Some(m) if self.0.iter().skip(1).all(|x| m == x.current()) => m,
            _ => None,
        }
    }

    fn running(&self) -> bool {
        self.0.iter().any(Automatic::running)
    }

    fn next(&mut self) {
        self.0.iter_mut().for_each(Automatic::next);
        self.0.retain(Automatic::running);
    }
}

#[derive(Clone)]
struct Buffered<T: Automatic + Clone>(Option<Movement>, T);

impl<T: Automatic + Clone> Buffered<T> {
    fn new(inner: T) -> Self {
        Self(inner.current(), inner)
    }
}

impl<T: Automatic + Clone> Automatic for Buffered<T> {
    fn current(&self) -> Option<Movement> {
        self.0
    }

    fn running(&self) -> bool {
        self.1.running()
    }

    fn next(&mut self) {
        self.1.next();
        self.0 = self.1.current();
    }
}
