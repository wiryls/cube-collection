use bevy::prelude::*;

/// Its lifetime is limited to a specific level.
#[derive(Component, bevy_inspector_egui::Inspectable)]
pub struct Live;

#[derive(Component, Clone, PartialEq)]
pub enum Type {
    White,
    Red,
    Blue,
    Green,
}

/// It is a grid point.
#[derive(Component, bevy_inspector_egui::Inspectable)]
pub struct Unit {
    pub x: i32,
    pub y: i32,
}

#[derive(Component, bevy_inspector_egui::Inspectable)]
pub struct Move {}

#[derive(Clone, PartialEq)]
pub enum Action {
    Idle,
    Left,
    Down,
    Up,
    Right,
}
