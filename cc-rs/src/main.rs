use bevy::prelude::*;
use iyes_loopless::prelude::*;

mod model;
mod scene;
mod extra;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        // plugins
        .add_plugin(extra::debug::DebugPlugin)
        // scenes
        .add_loopless_state(scene::State::default())
        .add_plugin(scene::LoadingScene)
        .add_plugin(scene::RunningScene)
        // done
        .run();
}
