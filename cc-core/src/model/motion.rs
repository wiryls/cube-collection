use std::sync::Arc;

use super::Movement;

/////////////////////////////////////////////////////////////////////////////
// export

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

#[derive(Clone, Debug)]
pub struct Motion(Any);

impl Motion {
    pub fn new() -> Self {
        Motion(Any::Stop)
    }

    pub fn from_sequence<I>(is_loop: bool, movements: I) -> Self
    where
        I: Iterator<Item = (Option<Movement>, usize)>,
    {
        Motion(Any::Move(Move {
            source: Arc::new(Sequence::new(is_loop, movements)),
            primary: 0,
            secondary: 0,
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

#[derive(Debug, Clone)]
struct Move {
    source: Arc<Sequence>,
    primary: usize,
    secondary: usize,
}

impl Iterator for Move {
    type Item = Option<Movement>;

    fn next(&mut self) -> Option<Self::Item> {
        let actions = self.source.actions.as_ref();

        let limit = actions.len();
        if self.primary == limit {
            return None;
        }

        let (movement, times) = actions[self.primary];
        self.secondary += 1;
        if self.secondary == times {
            self.secondary = 0;

            self.primary += 1;
            if self.primary == limit && self.source.looping {
                self.primary = 0;
            }
        }

        return Some(movement);
    }
}

#[derive(Clone, Debug)]
struct Team(Vec<Any>);

impl Iterator for Team {
    type Item = Option<Movement>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut vote = Agreement::new();
        self.0.retain_mut(|one| match one.next() {
            None => false,
            Some(choice) => {
                vote.submit(choice);
                true
            }
        });

        match self.0.len() {
            0 => None,
            _ => vote.result().or(Some(None)),
        }
    }
}

/////////////////////////////////////////////////////////////////////////////
// internal - Sequence

#[derive(Debug)]
struct Sequence {
    looping: bool,
    actions: Box<[(Option<Movement>, usize)]>,
}

impl Sequence {
    fn new(looping: bool, actions: impl Iterator<Item = (Option<Movement>, usize)>) -> Self {
        Self {
            looping,
            actions: actions.collect(),
        }
    }
}

/////////////////////////////////////////////////////////////////////////////
// tests

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
