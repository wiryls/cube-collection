use bevy::prelude::*;
use bevy::window::WindowResized;
use iyes_loopless::prelude::*;

mod grid;
mod location;
pub use grid::{GridView, ViewMapper};
pub use location::Location;

pub struct ViewUpdated {
    pub mapper: ViewMapper,
}

pub fn setup_adaptive_view(app: &mut App) {
    app.init_resource::<GridView>()
        .add_event::<ViewUpdated>()
        .add_startup_system(setup_camera)
        .add_system(update_gridview.run_on_event::<WindowResized>());
}

fn setup_camera(mut commands: Commands) {
    commands.spawn_bundle(Camera2dBundle::default());
}

fn update_gridview(
    windows: ResMut<Windows>,
    mut view: ResMut<GridView>,
    mut window_resized: EventReader<WindowResized>,
    mut mapper_updated: EventWriter<ViewUpdated>,
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
                mapper_updated.send(ViewUpdated { mapper });
            }
        }
    }
}
