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
pub struct Unit {

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

#[derive(Bundle)]
struct CubeBundle {
    live: Live,
    kind: Type,
}

#[derive(Bundle)]
struct UnitBundle {
    // ours
    style: Unit,
    point: GridPoint,
    // bevy's
    sprite: Sprite,
    transform: Transform,
    global_transform: GlobalTransform,
}
