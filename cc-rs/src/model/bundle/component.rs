use crate::model::common::Location;
use crate::model::unit;
use bevy::prelude::*;

/// Marks its lifetime is limited to a specific level.
#[derive(Component, Default)]
pub struct Earthbound;

/// The header of cubes.
#[derive(Component)]
pub struct Pack(pub unit::United);

impl From<unit::United> for Pack {
    fn from(united: unit::United) -> Self {
        Self(united)
    }
}

impl Location<i32> for Pack {
    fn x(&self) -> i32 {
        self.0.rect.left
    }
    fn y(&self) -> i32 {
        self.0.rect.top
    }
}

/// The actions list of cubes.
#[derive(Component)]
pub struct Move {/* TODO */}
