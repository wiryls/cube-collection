use std::{collections::HashMap, time::Duration};

use bevy::time::Timer;
use cube_core::{
    cube::{Movement, Point},
    seed::Seed,
    Diff, Unit,
};

pub struct World {
    state: cube_core::State,
    timer: Timer,
}

impl World {
    pub fn new(seed: &Seed) -> Self {
        Self {
            state: cube_core::State::new(&seed),
            timer: Timer::new(Duration::from_millis(200), true),
        }
    }

    pub fn next<T>(&mut self, delta: Duration, mut input: T) -> Option<HashMap<usize, Diff>>
    where
        T: FnMut() -> Option<Movement>,
    {
        if self.timer.tick(delta).finished() {
            self.state
                .commit(input())
                .map(|diff| (diff.id, diff))
                .collect::<HashMap<_, _, _>>()
                .into()
        } else {
            None
        }
    }

    pub fn cubes(&self) -> impl Iterator<Item = Unit> + '_ {
        self.state.iter()
    }

    pub fn goals(&self) -> impl Iterator<Item = Point> + '_ {
        self.state.goals().map(|(point, _)| point)
    }

    pub fn step(&self) -> Duration {
        self.timer.duration()
    }

    pub fn done(&self) -> bool {
        self.state.goals().all(|(_, ok)| ok)
    }

    pub fn width(&self) -> usize {
        self.state.width()
    }

    pub fn height(&self) -> usize {
        self.state.height()
    }
}
