use crate::model::{behavior, body, cube, seed};
use bevy::prelude::*;

/// Marks its lifetime is limited to a specific level.
#[derive(Component, Default)]
pub struct Earthbound;

/// The core of cubes.
#[derive(Component)]
pub struct CubeCore {
    pub kind: cube::Type,
    pub body: body::Unibody,
}

impl From<&seed::Cube> for CubeCore {
    fn from(cube: &seed::Cube) -> Self {
        Self {
            kind: cube.kind,
            body: body::Unibody::new(cube.body.iter()),
        }
    }
}

/// The actions list of cubes.
#[derive(Component)]
pub struct Movement(pub behavior::Behavior);
