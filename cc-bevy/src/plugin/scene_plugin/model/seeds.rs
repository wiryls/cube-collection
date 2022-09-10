use cc_core::seed::Seed;

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
}

impl From<Vec<Seed>> for Seeds {
    fn from(list: Vec<Seed>) -> Self {
        Self { list, head: 0 }
    }
}
