use super::super::view::GridView;
use super::{component::Cubic, world::World};
use bevy::prelude::*;

#[derive(Component, Default)]
pub struct Test;

pub fn movement(
    mut commands: Commands,
    mut cubes: Query<&mut Cubic>,
    mut world: ResMut<World>,
    view: ResMut<GridView>,
    time: Res<Time>,
) {
    let diffs = world.next(time.delta());
    if !diffs.is_empty() {
        let mapper = view.mapping();
        for mut cube in cubes.iter_mut() {
            // TODO: update cube by Diff
            todo!()
        }
    }
}
