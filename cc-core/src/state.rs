pub use super::model::{Diff, Item};
use super::{
    model::{Collection, Motion, Movement, Point, View},
    seed::{Cube, Kind, Seed},
};

pub struct State {
    mark: (usize, usize),
    dest: Vec<Point>,
    base: Collection,
    last: Option<Collection>,
    next: Option<Collection>,
}

impl State {
    pub fn new(seed: &Seed) -> Self {
        fn convert(cube: &Cube) -> (Kind, &[Point], Motion) {
            (
                cube.kind,
                cube.body.as_slice(),
                match &cube.command {
                    None => Motion::new(),
                    Some(command) => {
                        Motion::from_sequence(command.is_loop, command.movements.iter().cloned())
                    }
                },
            )
        }

        let dest = seed.destnations.clone();
        let mut base = Collection::new(
            seed.size.width.max(1) as usize,
            seed.size.height.max(1) as usize,
            seed.cubes.iter().map(convert),
        );
        base.preprocess();

        Self {
            mark: Self::calculate(&dest, &base.view()),
            dest,
            base,
            last: None,
            next: None,
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = Item> {
        self.latest().view().into_iter()
    }

    pub fn progress(&self) -> (usize, usize) {
        self.mark
    }

    pub fn input(&mut self, movement: Option<Movement>) -> impl Iterator<Item = Diff> {
        let mut next = self.base.clone();
        next.input(movement);
        self.last = self.next.replace(next);

        let last = self.last.as_ref().unwrap_or(&self.base).view();
        let next = self.next.as_ref().unwrap_or(&self.base).view();
        last.diff(next)
    }

    pub fn commit(&mut self) -> impl Iterator<Item = Diff> {
        if let Some(base) = self.next.take() {
            self.base = base;
        }

        let last = self.last.insert(self.base.clone()).view();
        self.base.postprocess();
        self.base.preprocess();
        let next = self.base.view();

        self.mark = Self::calculate(&self.dest, &next);
        last.diff(next)
    }

    fn latest(&self) -> &Collection {
        self.next.as_ref().unwrap_or(&self.base)
    }

    fn calculate(dest: &[Point], view: &View) -> (usize, usize) {
        let current = dest.iter().filter(|&&o| view.contains(o)).count();
        let total = dest.len();
        (current, total)
    }
}
