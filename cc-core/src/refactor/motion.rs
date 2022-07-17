use super::movement::Movement;

/////////////////////////////////////////////////////////////////////////////
// export

#[derive(Clone, Debug)]
pub struct Motion(Any);

#[allow(dead_code)]
impl Motion {
    pub fn new() -> Self {
        Motion(Any::Stop)
    }

    pub fn from_sequence<'a, I>(is_loop: bool, movements: I) -> Self
    where
        I: Iterator<Item = (Option<Movement>, usize)>,
    {
        Motion(Any::Move(Move {
            looping: is_loop,
            actions: movements.collect(),
            count: 0,
            index: 0,
        }))
    }

    pub fn from_iter<I>(others: I) -> Self
    where
        I: Iterator<Item = Self>,
    {
        let others = others
            .map(|x| x.0)
            .filter(|x| !matches!(x, Any::Stop))
            .collect::<Vec<_>>();

        Motion(Any::Team(Team(others)).slim())
    }

    pub fn r#take(&mut self) -> Self {
        Motion(self.take_inner())
    }

    fn take_inner(&mut self) -> Any {
        let mut that = Any::Stop;
        std::mem::swap(&mut that, &mut self.0);
        that
    }
}

impl Iterator for Motion {
    type Item = Option<Movement>;

    fn next(&mut self) -> Option<Self::Item> {
        self.0 = self.take_inner().slim();
        self.0.next()
    }
}

#[derive(Clone, Debug)]
pub struct Agreement(Option<Option<Option<Movement>>>);

impl Agreement {
    pub fn new() -> Self {
        Self(Some(None))
    }

    pub fn submit(&mut self, choice: Option<Movement>) {
        match self.0 {
            Some(Some(movement)) if movement != choice => self.0 = None,
            Some(None) => self.0 = Some(Some(choice)),
            _ => {}
        };
    }

    pub fn fail(&self) -> bool {
        self.0 == None
    }

    pub fn result(&self) -> Option<Option<Movement>> {
        self.0.flatten()
    }
}

/////////////////////////////////////////////////////////////////////////////
// internal

#[derive(Clone, Debug)]
enum Any {
    Stop,
    Move(Move),
    Team(Team),
}

impl Any {
    fn slim(self) -> Self {
        match self {
            Any::Team(x) if x.0.len() < 2 => match x.0.len() {
                0 => Any::Stop,
                1 => x.0.into_iter().next().unwrap(),
                _ => Any::Team(x),
            },
            _ => self,
        }
    }
}

impl Iterator for Any {
    type Item = Option<Movement>;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Any::Stop => None,
            Any::Move(x) => x.next(),
            Any::Team(x) => x.next(),
        }
    }
}

#[derive(Clone, Debug)]
struct Move {
    looping: bool,
    actions: Box<[(Option<Movement>, usize)]>,
    count: usize,
    index: usize,
}

impl Iterator for Move {
    type Item = Option<Movement>;

    fn next(&mut self) -> Option<Self::Item> {
        let n = self.actions.len();
        if self.index == n {
            return None;
        }

        let (o, m) = self.actions[self.index];
        self.count += 1;
        if self.count == m {
            self.index += 1;
            if self.index == n {
                if self.looping {
                    self.index = 0;
                }
            }
            self.count = 0;
        }

        return Some(o);
    }
}

#[derive(Clone, Debug)]
struct Team(Vec<Any>);

impl Iterator for Team {
    type Item = Option<Movement>;

    fn next(&mut self) -> Option<Self::Item> {
        // I need retain_mut

        let mut agreement = Agreement::new();
        let mut k = 0;
        for i in 0..self.0.len() {
            if let Some(choice) = self.0[i].next() {
                agreement.submit(choice);
                if k != i {
                    self.0.swap(i, k);
                }
                k += 1;
            }
        }
        self.0.truncate(k);

        match self.0.len() {
            0 => None,
            _ => agreement.result().or(Some(None)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn single_motion() {
        let mut stop = Motion::new();
        assert_eq!(stop.next(), None);

        let list = [(None, 2), (Some(Movement::Up), 2)];
        let mut list = Motion::from_sequence(false, list.into_iter());
        assert_eq!(list.next(), Some(None));
        assert_eq!(list.next(), Some(None));
        assert_eq!(list.next(), Some(Some(Movement::Up)));
        assert_eq!(list.next(), Some(Some(Movement::Up)));
        assert_eq!(list.next(), None);

        let list = [(Some(Movement::Left), 1), (Some(Movement::Up), 1)];
        let mut list = Motion::from_sequence(true, list.into_iter());
        assert_eq!(list.next(), Some(Some(Movement::Left)));
        assert_eq!(list.next(), Some(Some(Movement::Up)));
        assert_eq!(list.next(), Some(Some(Movement::Left)));
        assert_eq!(list.next(), Some(Some(Movement::Up)));
        assert_eq!(list.next(), Some(Some(Movement::Left)));
    }

    #[test]
    fn multiple_motion() {
        let team = [
            Motion::new(),
            Motion::from_sequence(false, [(None, 1)].into_iter()),
            Motion::from_sequence(false, [(None, 2)].into_iter()),
            Motion::from_sequence(false, [(None, 3)].into_iter()),
            Motion::from_sequence(false, [(None, 2), (Some(Movement::Up), 2)].into_iter()),
            Motion::new(),
            Motion::from_sequence(
                true,
                [(Some(Movement::Left), 1), (Some(Movement::Up), 1)].into_iter(),
            ),
        ];
        let mut team = Motion::from_iter(team.into_iter());
        assert_eq!(team.next(), Some(None));
        assert_eq!(team.next(), Some(None));
        assert_eq!(team.next(), Some(None));
        assert_eq!(team.next(), Some(Some(Movement::Up)));
        assert_eq!(team.next(), Some(Some(Movement::Left)));
        assert_eq!(team.next(), Some(Some(Movement::Up)));
        assert!(matches!(team, Motion(Any::Move(_))));
    }
}
