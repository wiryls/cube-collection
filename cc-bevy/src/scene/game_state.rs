#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GameState {
    Loading,
    Running,
}

impl Default for GameState {
    fn default() -> Self {
        Self::Loading
    }
}
