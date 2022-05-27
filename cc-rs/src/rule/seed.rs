use bevy::reflect::TypeUuid;

#[derive(Clone, Debug, Default, TypeUuid)]
#[uuid = "c99b1333-8ad3-4b26-a54c-7de542f43c51"]
pub struct Seed {
    pub info: Info,
    pub size: Size,
    pub cubes: Vec<Cube>,
    pub destnations: Vec<Location>,
}

#[derive(Clone, Debug, Default)]
pub struct Info {
    pub title: String,
    pub author: String,
}

#[derive(Clone, Debug, Default)]
pub struct Size {
    pub width: i32,
    pub height: i32,
}

#[derive(Clone, Debug, Default)]
pub struct Cube {
    pub kind: CubeType,
    pub body: Vec<Location>,
    pub command: Option<Command>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum CubeType {
    White,
    Red,
    Blue,
    Green,
}

impl Default for CubeType {
    fn default() -> Self {
        CubeType::White
    }
}

#[derive(Clone, Debug, Default)]
pub struct Location {
    pub x: i32,
    pub y: i32,
}

#[derive(Clone, Debug, Default)]
pub struct Command {
    pub is_loop: bool,
    pub movements: Vec<(i32, Movement)>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Movement {
    Idle,
    Left,
    Down,
    Up,
    Right,
}

impl Default for Movement {
    fn default() -> Self {
        Movement::Idle
    }
}
