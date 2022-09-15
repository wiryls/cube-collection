use super::{
    cube::{Kind, Motion, Movement, Point},
    rule::{Collection, Diff, Snapshot, Unit},
    seed::{Cube, Seed},
};

pub struct CubeCore {
    dest: Vec<Point>,
    last: Option<(Collection, Snapshot)>,
    base: (Collection, Snapshot),
}

impl CubeCore {
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
        let collection = Collection::new(
            seed.size.width.max(1) as usize,
            seed.size.height.max(1) as usize,
            seed.cubes.iter().map(convert),
        );
        let snapshot = collection.snapshot();

        Self {
            dest,
            last: None,
            base: (collection, snapshot),
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = Unit> + '_ {
        self.base.1.iter()
    }

    pub fn goals(&self) -> impl Iterator<Item = (Point, bool)> + '_ {
        self.dest.iter().map(|&o| (o, self.base.1.contains(o)))
    }

    pub fn commit(&mut self, movement: Option<Movement>) -> impl Iterator<Item = Diff> + '_ {
        let mut base = self.base.0.clone();
        base.commit(movement);
        let snapshot = base.snapshot();
        let mut base = (base, snapshot);

        std::mem::swap(&mut self.base, &mut base);
        let last = self.last.insert(base);
        last.1.differ(&self.base.1)
    }

    pub fn remake(&mut self, movement: Option<Movement>) -> impl Iterator<Item = Diff> + '_ {
        let pair = match &mut self.last {
            None => (&self.base.1, &self.base.1),
            Some(last) => {
                let mut base = last.0.clone();
                base.commit(movement);

                last.1 = base.snapshot();
                std::mem::swap(&mut self.base.1, &mut last.1);
                self.base.0 = base;

                (&last.1, &self.base.1)
            }
        };

        pair.0.differ(&pair.1)
    }

    pub fn width(&self) -> usize {
        self.base.0.width()
    }

    pub fn height(&self) -> usize {
        self.base.0.height()
    }
}
