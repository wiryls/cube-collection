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
        app.add_plugin(ShapePlugin)
            .add_state::<SceneState>()
            .configure_sets(
                (
                    RunningSet::Input,
                    RunningSet::Calculate,
                    RunningSet::Transform,
                )
                    .chain(),
            );

        view::setup(app, RunningSet::Input);
        input::setup(app, RunningSet::Input, SceneState::Running);

        scene_loading::setup(app);
        scene_running::setup(
            app,
            RunningSet::Input,
            RunningSet::Calculate,
            RunningSet::Transform,
        );
    }
}

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
#[system_set(base)]
enum RunningSet {
    Input,
    Calculate,
    Transform,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash, States)]
enum SceneState {
    #[default]
    Loading,
    Running,
}
