#[derive(Clone, Copy)]
pub enum Restriction {
    Free,
    Knock,
    Block,
}

impl Default for Restriction {
    fn default() -> Self {
        Self::Free
    }
}
