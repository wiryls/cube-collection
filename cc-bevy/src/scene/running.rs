use bevy::prelude::*;
use iyes_loopless::prelude::*;

use super::game_state::GameState;
use crate::model::{component, seed, system};
use crate::plugin::grid::{GridPlugin, GridUpdated, GridView};

/// - input: ```Res<seed::Seeds>```
/// - output: none
pub struct RunningScene;
impl Plugin for RunningScene {
    fn build(&self, app: &mut App) {
        app.add_plugin(GridPlugin)
            .add_event::<WorldChanged>()
            .add_enter_system(GameState::Running, setup_world)
            .add_system_set(
                ConditionSet::new()
                    .label("flow")
                    .run_in_state(GameState::Running)
                    .with_system(switch_world.run_on_event::<WorldChanged>())
                    .with_system(update_scale.run_on_event::<GridUpdated>())
                    .into(),
            )
            .add_system_set(
                ConditionSet::new()
                    .label("rule")
                    .after("flow")
                    .run_in_state(GameState::Running)
                    .with_system(system::movement.run_if_resource_exists::<seed::CubeWorld>())
                    .into(),
            );
    }
}

#[allow(unused)]
pub enum WorldChanged {
    Reset,
    Next,
}

fn setup_world(mut reset: EventWriter<WorldChanged>) {
    reset.send(WorldChanged::Reset)
}

fn switch_world(
    mut commands: Commands,
    entities: Query<Entity, With<component::Earthbound>>,
    mut view: ResMut<GridView>,
    mut world_seeds: ResMut<seed::CubeWorldSeeds>,
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
            // [0] update grid
            view.set_source(UiRect {
                left: 0,
                right: seed.width(),
                top: 0,
                bottom: seed.height(),
            });

            // [1] remove old objects
            entities.for_each(|i| commands.entity(i).despawn_recursive());

            // [2] create new cubes
            let mapper = view.mapping();
            let world = seed::CubeWorld::new(&seed);
            component::spawn_cubes(&world, &mut commands, &mapper);
            commands.insert_resource(world);
        }
    }
}

fn update_scale(
    mut cubes: Query<(&component::Cubic, &mut Transform)>,
    mut grid_updated: EventReader<GridUpdated>,
) {
    let event = match grid_updated.iter().last() {
        None => return,
        Some(event) => event,
    };

    let grid = &event.mapper;
    let scale = grid.scale(1.0);
    for (cube, mut transform) in cubes.iter_mut() {
        transform.translation = grid.absolute(&cube.position).extend(0.);
        transform.scale = Vec3::new(scale, scale, 1.0);
    }
}
