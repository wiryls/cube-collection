use bevy::prelude::*;
use iyes_loopless::prelude::*;

use super::cube::{component, system, world};
use super::view::{GridView, ViewUpdated};
use super::SceneState;

pub fn setup(appx: &mut App, prepare: impl StageLabel, rule: impl StageLabel) {
    appx.add_event::<WorldChanged>()
        .add_enter_system(SceneState::Running, setup_world)
        .add_system_set_to_stage(
            prepare,
            ConditionSet::new()
                .run_in_state(SceneState::Running)
                .with_system(switch_world.run_on_event::<WorldChanged>())
                .with_system(update_scale.run_on_event::<ViewUpdated>())
                .into(),
        )
        .add_system_to_stage(
            rule,
            system::state
                .run_in_state(SceneState::Running)
                .run_if_resource_exists::<world::World>(),
        )
        .add_system_set(
            ConditionSet::new()
                .run_in_state(SceneState::Running)
                .with_system(system::position)
                .with_system(system::recolor)
                .with_system(system::reshape)
                .into(),
        );
}

pub enum WorldChanged {
    Reset,
    Restart,
    Next,
}

fn setup_world(mut change_world: EventWriter<WorldChanged>) {
    change_world.send(WorldChanged::Reset)
}

fn switch_world(
    mut commands: Commands,
    entities: Query<Entity, With<component::Earthbound>>,
    mut view: ResMut<GridView>,
    mut world_seeds: ResMut<world::Seeds>,
    mut world_changed: EventReader<WorldChanged>,
) {
    let got = !world_changed.is_empty();
    for event in world_changed.iter() {
        use WorldChanged::*;
        match event {
            Reset => world_seeds.reset(),
            Restart => {}
            Next => drop(world_seeds.next()),
        }
    }

    if let Some(seed) = got.then(|| world_seeds.current()).flatten() {
        // [0] remove all old objects
        entities.for_each(|i| commands.entity(i).despawn_recursive());

        // [1] update grid
        view.set_source(UiRect {
            left: 0,
            right: seed.size.width,
            top: 0,
            bottom: seed.size.height,
        });
        let mapper = view.mapping();

        // [2] create new world
        let world = world::World::new(&seed);
        component::spawn_objects(&world, &mut commands, &mapper);
        commands.insert_resource(world);
    }
}

fn update_scale(
    mut cubes: Query<(&component::Cubic, &mut Transform)>,
    mut grid_updated: EventReader<ViewUpdated>,
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
