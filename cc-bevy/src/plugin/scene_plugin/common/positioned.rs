use bevy::prelude::*;
use cc_core::cube::Point;

use super::super::view::ViewUpdated;

/// Gridded marks that the current object scales with grid size. Rescaling
/// happends when windows resized.
#[derive(Component)]
pub struct Gridded {
    pub point: Point,
}

impl From<Point> for Gridded {
    fn from(point: Point) -> Self {
        Self { point }
    }
}

impl From<Gridded> for Point {
    fn from(gridded: Gridded) -> Self {
        gridded.point
    }
}

/// Rescale Gridded entities when windows resized.
pub fn gridded_system(
    mut cubes: Query<(&Gridded, &mut Transform)>,
    mut view_updated: EventReader<ViewUpdated>,
) {
    let event = match view_updated.iter().last() {
        None => return,
        Some(event) => event,
    };

    let grid = &event.mapper;
    let scale = grid.scale(1.0);
    for (gridded, mut transform) in cubes.iter_mut() {
        let z = transform.translation.z;
        transform.translation = grid.absolute(&gridded.point).extend(z);
        transform.scale = Vec3::new(scale, scale, 1.0);
    }
}
