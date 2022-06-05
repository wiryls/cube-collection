use super::{behavior, common, cube};
use bevy::reflect::TypeUuid;

pub struct Seeds {
    list: Vec<Seed>,
    head: usize,
}

impl Seeds {
    pub fn current(&self) -> Option<&Seed> {
        self.list.get(self.head)
    }

    pub fn next(&mut self) -> bool {
        self.head += 1;
        if self.head >= self.list.len() {
            self.head = 0;
            false
        } else {
            true
        }
    }
}

impl From<Vec<Seed>> for Seeds {
    fn from(seeds: Vec<Seed>) -> Self {
        Self {
            list: seeds,
            head: 0,
        }
    }
}

#[derive(Clone, TypeUuid)]
#[uuid = "c99b1333-8ad3-4b26-a54c-7de542f43c51"]
pub struct Seed {
    pub info: Info,
    pub size: Size,
    pub cubes: Vec<Cube>,
    pub destnations: Vec<Location>,
}

#[derive(Clone, Default)]
pub struct Info {
    pub title: String,
    pub author: String,
}

#[derive(Clone)]
pub struct Size {
    pub width: i32,
    pub height: i32,
}

#[derive(Clone)]
pub struct Cube {
    pub kind: cube::Type,
    pub body: Vec<Location>,
    pub command: Option<Command>,
}

#[derive(Clone)]
pub struct Location {
    pub x: i32,
    pub y: i32,
}

impl common::Location<i32> for Location {
    fn x(&self) -> i32 {
        self.x
    }
    fn y(&self) -> i32 {
        self.y
    }
}

#[derive(Clone, Default)]
pub struct Command {
    pub is_loop: bool,
    pub movements: Vec<(behavior::Movement, usize)>,
}
