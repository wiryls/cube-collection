use bevy::prelude::*;

#[derive(Component, bevy_inspector_egui::Inspectable)]
pub struct Live;

#[derive(Component, bevy_inspector_egui::Inspectable)]
pub struct GridPoint {
    pub x: i32,
    pub y: i32,
}

#[derive(Component, Clone, PartialEq)]
pub enum Type {
    White,
    Red,
    Blue,
    Green,
}

#[derive(Component)]
struct Group {
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
