use std::{collections::HashMap, time::Duration};

use bevy::{reflect::TypeUuid, time::Timer};
use cc_core::{cube::Movement, seed::Seed, Diff};

#[derive(Clone, TypeUuid)]
#[uuid = "c99b1333-8ad3-4b26-a54c-7de542f43c51"]
pub struct CubeWorldSeed(pub Seed);

impl CubeWorldSeed {
    pub fn new(seed: Seed) -> Self {
        Self(seed)
    }

    pub fn height(&self) -> i32 {
        self.0.size.height
    }

    pub fn width(&self) -> i32 {
        self.0.size.width
    }
}

pub struct CubeWorldSeeds {
    list: Vec<CubeWorldSeed>,
    head: usize,
}

impl CubeWorldSeeds {
    pub fn current(&self) -> Option<&CubeWorldSeed> {
        self.list.get(self.head)
    }

    pub fn next(&mut self) -> bool {
        self.head += 1;
        if self.head >= self.list.len() {
            self.head = 0;
            false
        } else {
            true
        }
    }
}

impl From<Vec<CubeWorldSeed>> for CubeWorldSeeds {
    fn from(seeds: Vec<CubeWorldSeed>) -> Self {
        Self {
            list: seeds,
            head: 0,
        }
    }
}

pub struct CubeWorld {
    state: cc_core::State,
    timer: Timer,
}

impl CubeWorld {
    pub fn new(seed: &CubeWorldSeed) -> Self {
        Self {
            state: cc_core::State::new(&seed.0),
            timer: Timer::new(Duration::from_millis(200), true),
        }
    }

    pub fn next(&mut self, delta: Duration) -> HashMap<usize, Diff> {
        let mut output = HashMap::new();
        if self.timer.tick(delta).finished() {
            self.state.commit(None);
        }

        output
    }

    pub fn cubes(&self) -> impl Iterator<Item = cc_core::Unit> + '_ {
        self.state.iter()
    }
}

pub enum Command {
    Left,
    Down,
    Up,
    Right,
}

impl From<Command> for Movement {
    fn from(command: Command) -> Self {
        match command {
            Command::Left => Movement::Left,
            Command::Down => Movement::Down,
            Command::Up => Movement::Up,
            Command::Right => Movement::Right,
        }
    }
}
