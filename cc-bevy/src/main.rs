use bevy::prelude::*;

mod plugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        // optional plugins
        .add_plugin(plugin::DebugPlugin)
        // scenes
        .add_plugin(plugin::ScenePlugin)
        // done
        .run();
}
