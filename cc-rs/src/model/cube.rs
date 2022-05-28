use bevy::prelude::*;

#[derive(Component, bevy_inspector_egui::Inspectable)]
pub struct Gridded{
    pub x: i32,
    pub y: i32,
}
