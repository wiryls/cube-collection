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
                    RunningSet::Prepare,
                    RunningSet::Input,
                    RunningSet::Calculate,
                )
                    .chain(),
            );

        view::setup(app, RunningSet::Input);
        input::setup(app, RunningSet::Input, SceneState::Running);

        scene_loading::setup(app);
        scene_running::setup(
            app,
            RunningSet::Prepare,
            RunningSet::Input,
            RunningSet::Calculate,
        );
    }
}

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
#[system_set(base)]
enum RunningSet {
    Prepare,
    Input,
    Calculate,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash, States)]
enum SceneState {
    #[default]
    Loading,
    Running,
}
