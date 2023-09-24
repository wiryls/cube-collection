use std::{collections::HashMap, time::Duration};

use bevy::prelude::*;
use bevy::time::Timer;
use cube_core::{
    cube::{Movement, Point},
    seed::Seed,
    Diff, Unit,
};

#[derive(Resource)]
pub struct World {
    state: cube_core::CubeCore,
    timer: Timer,
}

impl World {
    pub fn new(seed: &Seed) -> Self {
        Self {
            state: cube_core::CubeCore::new(&seed),
            timer: Timer::new(Duration::from_millis(200), TimerMode::Repeating),
        }
    }

    pub fn next(&mut self, movement: Option<Movement>) -> HashMap<usize, Diff> {
        self.state
            .commit(movement)
            .map(|diff| (diff.id, diff))
            .collect::<HashMap<_, _, _>>()
            .into()
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
