use bevy::prelude::*;

mod plugin;

fn windows_settings() -> WindowDescriptor {
    WindowDescriptor {
        title: "Cube Collection".to_owned(),
        ..Default::default()
    }
}

fn main() {
    App::new()
        .insert_resource(windows_settings())
        .add_plugins(DefaultPlugins)
        // scenes
        .add_plugin(plugin::ScenePlugin)
        // done
        .run();
}
