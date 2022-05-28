use bevy::prelude::*;

/// Its lifetime is limited to a specific level.
#[derive(Component, bevy_inspector_egui::Inspectable)]
pub struct Live;

/// It has a point
#[derive(Component, bevy_inspector_egui::Inspectable)]
pub struct Unit {

}

#[derive(Component, bevy_inspector_egui::Inspectable)]
pub struct Move {

}

/// 
#[derive(Component, bevy_inspector_egui::Inspectable)]
pub struct Gridded{
    pub x: i32,
    pub y: i32,
}
