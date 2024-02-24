use bevy::prelude::*;

use crate::plugin::ShapePlugin;

mod common;
mod input;
mod model;
mod scene_loading;
mod scene_running;
mod view;

pub struct ScenePlugin;
impl Plugin for ScenePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(ShapePlugin).init_state::<SceneState>();

        view::setup(app);
        input::setup(app, SceneState::Running);
        scene_loading::setup(app);
        scene_running::setup(app);
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash, States)]
enum SceneState {
    #[default]
    Loading,
    Running,
}
