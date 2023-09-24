use bevy::prelude::*;

mod plugin;

fn windows_settings() -> WindowPlugin {
    WindowPlugin {
        primary_window: Some(Window {
            title: "Cube Collection".to_owned(),
            ..Default::default()
        }),
        ..default()
    }
}

fn main() {
    let plugins = (DefaultPlugins.set(windows_settings()), plugin::ScenePlugin);
    App::new().add_plugins(plugins).run();
}
