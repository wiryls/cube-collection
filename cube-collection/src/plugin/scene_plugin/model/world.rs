use std::collections::HashMap;

use bevy::prelude::*;
use cube_core::{
    cube::{Movement, Point},
    seed::Seed,
    Diff, Unit,
};

#[derive(Resource)]
pub struct World(cube_core::CubeCore);

impl World {
    pub fn new(seed: &Seed) -> Self {
        Self(cube_core::CubeCore::new(&seed))
    }

    pub fn next(&mut self, movement: Option<Movement>) -> HashMap<usize, Diff> {
        self.0
            .commit(movement)
            .map(|diff| (diff.id, diff))
            .collect::<HashMap<_, _, _>>()
            .into()
    }

    pub fn cubes(&self) -> impl Iterator<Item = Unit> + '_ {
        self.0.iter()
    }

    pub fn goals(&self) -> impl Iterator<Item = Point> + '_ {
        self.0.goals().map(|(point, _)| point)
    }

    pub fn done(&self) -> bool {
        self.0.goals().all(|(_, ok)| ok)
    }

    pub fn width(&self) -> usize {
        self.0.width()
    }

    pub fn height(&self) -> usize {
        self.0.height()
    }
}
