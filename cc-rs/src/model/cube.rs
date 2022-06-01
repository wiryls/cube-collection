use super::{common, detail};
use bevy::prelude::*;

/// Marks its is a lattice grid and will be rescale when window size changed.
#[derive(Component)]
pub struct GridPoint(pub common::Point);

impl common::Location<i32> for GridPoint {
    fn x(&self) -> i32 {
        self.0.x
    }

    fn y(&self) -> i32 {
        self.0.y
    }
}

impl<T: common::Location<i32>> From<&T> for GridPoint {
    fn from(location: &T) -> Self {
        Self(common::Point::new(location.x(), location.y()))
    }
}

/// Marks its lifetime is limited to a specific level.
#[derive(Component)]
pub struct Live;

/// The type of cubes.
#[derive(Component, Clone, Copy, PartialEq)]
pub enum Type {
    White,
    Red,
    Blue,
    Green,
}

/// The header of cubes.
#[derive(Component)]
pub struct Pack(pub detail::United);

impl From<detail::United> for Pack {
    fn from(united: detail::United) -> Self {
        Self(united)
    }
}

/// The actions list of cubes.
#[derive(Component)]
pub struct Move {/* TODO */}

/// Cube's current action.
#[derive(Clone, PartialEq)]
pub enum Action {
    Idle,
    Left,
    Down,
    Up,
    Right,
}
