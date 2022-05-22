use bevy::{prelude::*, window::WindowResized};
use super::{GridLocator, LocatorUpdated};

pub struct ViewPlugin;

impl Plugin for ViewPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app .init_resource::<GridLocator>() 
            .add_event::<LocatorUpdated>()
            .add_startup_system(setup_camera)
            .add_system(window_resized)
            ;
    }
}

fn setup_camera(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}

fn window_resized(
    mut locator: ResMut<GridLocator>,
    mut window_resized: EventReader<WindowResized>,
    mut locator_updated: EventWriter<LocatorUpdated>
) {
    for e in window_resized.iter().last() {
        // our rect is defined by ScalingMode::WindowSize of camera
        let w = e.width / 2.;
        let h = e.height / 2.;
        let r = Rect {left: -w, right: w, top: h, bottom: -h};
        if locator.set_view(r) && locator.available() {
            locator_updated.send(LocatorUpdated{mapper: locator.mapping().clone()})
        }
    }
}
