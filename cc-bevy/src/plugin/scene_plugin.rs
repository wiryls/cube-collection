use bevy::prelude::*;

mod input;
mod rule;
mod scene_loading;
mod scene_running;
mod view;

struct Lable;
impl Lable {
    pub const VIEW: &'static str = "view";
    pub const INPUT: &'static str = "input";
    pub const LOADING: &'static str = "loading";
    pub const RUNNING: &'static str = "running";
}

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
        view::setup(app);
        input::setup(app);
        scene_loading::setup(app);
        scene_running::setup(app);
    }
}
