use super::cube::{Kind, Movement, Point};

#[derive(Clone, Debug)]
pub struct Seed {
    pub info: Info,
    pub size: Size,
    pub cubes: Vec<Cube>,
    pub destinations: Vec<Point>,
}

#[derive(Clone, Debug)]
pub struct Info {
    pub title: String,
    pub author: String,
}

#[derive(Clone, Debug)]
pub struct Cube {
    pub kind: Kind,
    pub body: Vec<Point>,
    pub command: Option<Command>,
}

#[derive(Clone, Debug)]
pub struct Size {
    pub width: i32,
    pub height: i32,
}

#[derive(Clone, Debug)]
pub struct Command {
    pub is_loop: bool,
    pub movements: Vec<(Option<Movement>, usize)>,
}
