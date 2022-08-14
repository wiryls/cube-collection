use super::model::{Kind, Movement, Point};

#[derive(Clone)]
pub struct Seed {
    pub info: Info,
    pub size: Size,
    pub cubes: Vec<Cube>,
    pub destnations: Vec<Point>,
}

#[derive(Clone)]
pub struct Info {
    pub title: String,
    pub author: String,
}

#[derive(Clone)]
pub struct Cube {
    pub kind: Kind,
    pub body: Vec<Point>,
    pub command: Option<Command>,
}

#[derive(Clone)]
pub struct Size {
    pub width: i32,
    pub height: i32,
}

#[derive(Clone)]
pub struct Command {
    pub is_loop: bool,
    pub movements: Vec<(Option<Movement>, usize)>,
}
