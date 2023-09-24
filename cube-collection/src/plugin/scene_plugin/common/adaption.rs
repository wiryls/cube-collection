use bevy::prelude::*;
use cube_core::cube::Point;

use super::super::view::ViewUpdated;

/// AutoRescale marks that the current object fixed at a grid point and scales
/// with grid size. Relocating happens when windows get resized.
#[derive(Component)]
pub struct AutoRescale {
    pub point: Point,
    pub offset: f32,
}

/// Rescale and relocate entities when windows resized.
pub fn self_adaption_system(
    mut query: Query<(&mut Transform, &AutoRescale)>,
    mut view_updated: EventReader<ViewUpdated>,
) {
    let event = match view_updated.iter().last() {
        None => return,
        Some(event) => event,
    };

    let mapper = &event.mapper;
    let scale = Vec3::new(mapper.unit(), mapper.unit(), 1.0);
    for (mut transform, relocate) in &mut query {
        let z = transform.translation.z;
        let v = mapper.locate(&relocate.point) + mapper.scale(&(relocate.offset, relocate.offset));
        transform.translation = v.extend(z);
        transform.scale = scale;
    }
}
