use bevy::prelude::*;
use iyes_loopless::prelude::*;

use super::state::State;
use crate::extra::grid::{GridPlugin, GridUpdated, GridView};
use crate::model::{bundle, seed};

/// - input: ```Res<seed::Seeds>```
/// - output: none
pub struct RunningScene;
impl Plugin for RunningScene {
    fn build(&self, app: &mut App) {
        app.add_plugin(GridPlugin)
            .add_event::<WorldChanged>()
            .add_enter_system(State::Running, setup_world)
            .add_system_set(
                ConditionSet::new()
                    .run_in_state(State::Running)
                    .with_system(switch_world.run_on_event::<WorldChanged>())
                    .with_system(regrid.run_on_event::<GridUpdated>())
                    .into(),
            );
    }
}

pub enum WorldChanged {
    Reset,
    Next,
}

fn setup_world(mut reset: EventWriter<WorldChanged>) {
    reset.send(WorldChanged::Reset)
}

fn switch_world(
    mut commands: Commands,
    entities: Query<Entity, With<bundle::Earthbound>>,
    mut view: ResMut<GridView>,
    mut world_seeds: ResMut<seed::Seeds>,
    mut world_changed: EventReader<WorldChanged>,
) {
    let got = !world_changed.is_empty();
    for event in world_changed.iter() {
        if let WorldChanged::Next = event {
            world_seeds.next();
        }
    }

    match got.then(|| world_seeds.current()).flatten() {
        None => return,
        Some(seed) => {
            // [0] update grid view
            view.set_source(Rect {
                left: 0,
                right: seed.size.width,
                top: 0,
                bottom: seed.size.height,
            });

            // [1] remove old objects
            entities.for_each(|i| commands.entity(i).despawn_recursive());

            // [2] create new cubes
            let mapper = view.mapping();
            for c in &seed.cubes {
                bundle::spawn_cube(&c, &mut commands, &mapper);
            }

            // for o in &seed.destnations {
            //     commands
            //         .spawn_bundle(SpriteBundle {
            //             sprite: Sprite {
            //                 color: Color::rgb(0.1, 0.1, 0.1),
            //                 ..default()
            //             },
            //             transform: Transform {
            //                 scale: Vec3::new(scale, scale, 1.),
            //                 translation: mapper.locate(o.x, o.y, 0.),
            //                 ..default()
            //             },
            //             ..default()
            //         })
            //         .insert(cube::GridPoint { x: o.x, y: o.y })
            //         .insert(cube::Live);
            // }
        }
    }
}

fn regrid(
    mut cubes: Query<(&bundle::CubeCore, &mut Transform)>,
    mut grid_updated: EventReader<GridUpdated>,
) {
    let event = match grid_updated.iter().last() {
        None => return,
        Some(event) => event,
    };

    let grid = &event.mapper;
    let scale = grid.scale(1.0);
    for (cube, mut transform) in cubes.iter_mut() {
        transform.translation = grid.absolute(cube).extend(0.0);
        transform.scale = Vec3::new(scale, scale, 1.0);
    }
}
