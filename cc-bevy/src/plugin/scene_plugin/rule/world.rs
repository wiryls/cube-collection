use bevy::time::Timer;
use cc_core::{cube::Movement, seed::Seed, Diff};
use std::{collections::HashMap, time::Duration};

pub struct Seeds {
    list: Vec<Seed>,
    head: usize,
}

impl Seeds {
    pub fn current(&self) -> Option<&Seed> {
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

impl From<Vec<Seed>> for Seeds {
    fn from(seeds: Vec<Seed>) -> Self {
        Self {
            list: seeds,
            head: 0,
        }
    }
}

pub struct World {
    state: cc_core::State,
    timer: Timer,
}

impl World {
    pub fn new(seed: &Seed) -> Self {
        Self {
            state: cc_core::State::new(&seed),
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
