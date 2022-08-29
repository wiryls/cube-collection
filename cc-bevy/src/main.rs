use bevy::prelude::*;
use iyes_loopless::prelude::*;

mod model;
mod plugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        // optional plugins
        .add_plugin(plugin::DebugPlugin)
        // scenes
        .add_loopless_state(plugin::SceneState::default())
        .add_plugin(plugin::ScenePlugin)
        // done
        .run();
}
