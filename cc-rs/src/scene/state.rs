#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum State {
    Loading,
    Running,
}

impl Default for State {
    fn default() -> Self {
        Self::Loading
    }
}
