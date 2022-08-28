mod location;
mod view;

pub use location::Location;
pub use view::{GridMapper, GridView};

use bevy::prelude::*;
use bevy::window::WindowResized;
use iyes_loopless::prelude::*;

pub struct GridUpdated {
    pub mapper: GridMapper,
}

// GridPlugin adds a GridView resource and a GridUpdated event.
pub struct GridPlugin;
impl Plugin for GridPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.init_resource::<GridView>()
            .add_event::<GridUpdated>()
            .add_startup_system(setup_camera)
            .add_system(window_resized.run_on_event::<WindowResized>());
    }
}

fn setup_camera(mut commands: Commands) {
    commands.spawn_bundle(Camera2dBundle::default());
}

fn window_resized(
    windows: ResMut<Windows>,
    mut view: ResMut<GridView>,
    mut window_resized: EventReader<WindowResized>,
    mut mapper_updated: EventWriter<GridUpdated>,
) {
    if let Some(window) = windows.get_primary() {
        // multiple windows are not supported, we just watch the primary one.
        let id = window.id();

        for event in window_resized.iter().filter(|x| x.id == id).last() {
            // rect is defined by camera's ScalingMode::WindowSize
            let w = event.width / 2.;
            let h = event.height / 2.;
            let r = UiRect {
                left: -w,
                right: w,
                top: h,
                bottom: -h,
            };

            if view.set_target(r) && view.available() {
                let mapper = view.mapping().clone();
                mapper_updated.send(GridUpdated { mapper });
            }
        }
    }
}
