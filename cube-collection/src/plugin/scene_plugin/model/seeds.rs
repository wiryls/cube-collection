use bevy::prelude::*;
use cube_core::seed::Seed;

#[derive(Resource)]
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
}

impl From<Vec<Seed>> for Seeds {
    fn from(list: Vec<Seed>) -> Self {
        Self { list, head: 0 }
    }
}
