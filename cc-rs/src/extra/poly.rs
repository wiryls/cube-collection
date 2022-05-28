use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

/// A wrapper of ShapePlugin.
pub struct PolyPlugin;
impl Plugin for PolyPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.insert_resource(Msaa { samples: 4 })
            .add_plugin(ShapePlugin);
    }
}
