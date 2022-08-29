use bevy::prelude::*;
use bevy_prototype_lyon::plugin;

/// A wrapper of ShapePlugin.
pub struct ShapePlugin;
impl Plugin for ShapePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Msaa { samples: 4 })
            .add_plugin(plugin::ShapePlugin);
    }
}
