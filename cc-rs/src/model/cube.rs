use super::detail;
use bevy::prelude::*;

/// Marks its is a lattice grid and will be rescale when window size changed.
#[derive(Component, bevy_inspector_egui::Inspectable)]
pub struct GridPoint {
    pub x: i32,
    pub y: i32,
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

#[derive(Component)]
pub struct Pattern(u8);

impl From<&detail::Near> for Pattern {
    fn from(near: &detail::Near) -> Self {
        Self(near.0)
    }
}

impl From<&Pattern> for detail::Near {
    fn from(pattern: &Pattern) -> Self {
        Self(pattern.0)
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
