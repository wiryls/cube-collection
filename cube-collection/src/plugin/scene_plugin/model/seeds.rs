use bevy::prelude::*;
use cube_core::cube::{Kind, Point};
use cube_core::seed::{Cube, Info, Seed, Size};

#[derive(Resource, Debug)]
pub struct Seeds {
    list: Vec<Seed>,
    head: usize,
}

impl Seeds {
    pub fn current(&self) -> Option<&Seed> {
        self.list.get(self.head)
    }

    pub fn reset(&mut self) {
        self.head = 0;
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

    pub fn last(&mut self) -> bool {
        if self.head == 0 {
            self.head = self.list.len().max(1) - 1;
            false
        } else {
            self.head -= 1;
            true
        }
    }

    pub fn error() -> Self {
        const ART: &str = "
.###.##..##..###.##..
.#...#.#.#.#.#.#.#.#.
.##..##..##..#.#.##..
.#...#.#.#.#.#.#.#.#.
.###.#.#.#.#.###.#.#.

";

        let mut cubes = Vec::new();
        for (y, row) in ART.lines().enumerate() {
            for (x, ch) in row.chars().enumerate() {
                if ch == '#' {
                    cubes.push(Cube {
                        kind: Kind::Red,
                        body: vec![Point::new(x as i32, y as i32)],
                        command: None,
                    });
                }
            }
        }

        let seed = Seed {
            info: Info {
                title: "ERROR".into(),
                author: String::new(),
            },
            size: Size {
                width: ART.lines().map(|x| x.len() as i32).max().unwrap_or(1),
                height: ART.lines().count() as i32,
            },
            cubes,
            destinations: vec![],
        };

        Self::from(vec![seed])
    }
}

impl From<Vec<Seed>> for Seeds {
    fn from(list: Vec<Seed>) -> Self {
        Self { list, head: 0 }
    }
}
