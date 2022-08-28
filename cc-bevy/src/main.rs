use bevy::prelude::*;
use iyes_loopless::prelude::*;

mod model;
mod plugin;
mod scene;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        // plugins
        .add_plugin(plugin::DebugPlugin)
        .add_plugin(plugin::PolyPlugin)
        // scenes
        .add_loopless_state(scene::GameState::default())
        .add_plugin(scene::LoadingScene)
        .add_plugin(scene::RunningScene)
        // done
        .run();
}
