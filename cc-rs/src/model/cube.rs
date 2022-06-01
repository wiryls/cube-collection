use super::detail;
use bevy::prelude::*;

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
