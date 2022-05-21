use bevy::{prelude::*, window::WindowResized};
use super::{Relocator, RelocatorUpdated};

pub struct ViewPlugin;

impl Plugin for ViewPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app .init_resource::<Relocator>() 
            .add_event::<RelocatorUpdated>()
            .add_startup_system(setup_camera)
            .add_system(on_window_resized)
            ;
    }
}

fn setup_camera(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}

fn on_window_resized(
    mut relocator: ResMut<Relocator>,
    mut window_resized: EventReader<WindowResized>,
    mut relocator_updated: EventWriter<RelocatorUpdated>
) {
    for e in window_resized.iter().last() {
        let rect = window_size_to_rect(e.width, e.height);
        if relocator.set_view(rect) && relocator.is_ready() {
            relocator_updated.send(RelocatorUpdated{mapper: relocator.mapping().clone()})
        }
    }
}

fn window_size_to_rect(width: f32, height: f32) -> Rect<f32> {
    // our rect is defined by ScalingMode::WindowSize of camera
    let w = width / 2.;
    let h = height / 2.;
    Rect {left: -w, right: w, top: h, bottom: -h}
}
