use bevy::prelude::*;

mod plugin;

fn windows_settings() -> WindowPlugin {
    WindowPlugin {
        window: WindowDescriptor {
            title: "Cube Collection".to_owned(),
            ..Default::default()
        },
        ..default()
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(windows_settings()))
        .add_plugin(plugin::ScenePlugin)
        .run();
}
