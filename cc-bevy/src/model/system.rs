use super::{component::Cubic, seed::CubeWorld};
use crate::extra::grid::GridView;
use bevy::prelude::*;

#[derive(Component, Default)]
pub struct Test;

pub fn movement(
    mut commands: Commands,
    mut cubes: Query<&mut Cubic>,
    mut world: ResMut<CubeWorld>,
    view: ResMut<GridView>,
    time: Res<Time>,
) {
    if world.tick(time.delta()) {
        let mapper = view.mapping();

        for mut cube in cubes.iter_mut() {
            // TODO: update cube by Diff
        }
    }
}
