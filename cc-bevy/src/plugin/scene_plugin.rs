use bevy::prelude::*;

mod rule;
mod scene_loading;
mod scene_running;
mod view;

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
        scene_loading::setup_scene(app);
        scene_running::setup_scene(app);
    }
}
