use crate::model::common::Location;
use crate::model::{cube, seed, unit};
use bevy::prelude::*;

/// Marks its lifetime is limited to a specific level.
#[derive(Component, Default)]
pub struct Earthbound;

/// The core of cubes.
#[derive(Component)]
pub struct Cube {
    pub kind: cube::Type,
    pub body: unit::Unibody,
}

impl From<&seed::Cube> for Cube {
    fn from(cube: &seed::Cube) -> Self {
        Self {
            kind: cube.kind,
            body: unit::Unibody::from(cube.body.iter()),
        }
    }
}

impl Location<i32> for Cube {
    fn x(&self) -> i32 {
        self.body.rect.left
    }
    fn y(&self) -> i32 {
        self.body.rect.top
    }
}

/// The actions list of cubes.
#[derive(Component)]
pub struct Move {/* TODO */}
