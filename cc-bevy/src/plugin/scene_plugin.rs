use bevy::prelude::*;

mod loading;
mod running;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SceneState {
    Loading,
    Running,
}

impl Default for SceneState {
    fn default() -> Self {
        Self::Loading
    }
}

pub struct ScenePlugin;
impl Plugin for ScenePlugin {
    fn build(&self, app: &mut App) {
        loading::setup_scene(app);
        running::setup_scene(app);
    }
}
