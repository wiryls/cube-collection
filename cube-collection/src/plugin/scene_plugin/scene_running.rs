use bevy::prelude::*;
use iyes_loopless::prelude::*;

use super::{
    common::{bundle, component, system},
    model,
    view::{GridView, ViewUpdated},
    SceneState,
};

pub fn setup(appx: &mut App, prepare: impl StageLabel, calculate: impl StageLabel) {
    appx.add_event::<WorldChanged>()
        .add_enter_system(SceneState::Running, setup_world)
        .add_system_set_to_stage(
            prepare,
            ConditionSet::new()
                .run_in_state(SceneState::Running)
                .with_system(switch_world.run_on_event::<WorldChanged>())
                .with_system(system::self_adaption.run_on_event::<ViewUpdated>())
                .into(),
        )
        .add_system_to_stage(
            calculate,
            system::state
                .run_in_state(SceneState::Running)
                .run_if_resource_exists::<model::World>(),
        )
        .add_system_set(
            ConditionSet::new()
                .run_in_state(SceneState::Running)
                .run_if_resource_exists::<model::World>()
                .with_system(system::position)
                .with_system(system::realpha)
                .with_system(system::recolor)
                .with_system(system::reshape)
                .into(),
        );
}

pub enum WorldChanged {
    Reset,
    Restart,
    Next,
    Last,
}

fn setup_world(mut change_world: EventWriter<WorldChanged>) {
    change_world.send(WorldChanged::Restart)
}

fn switch_world(
    mut commands: Commands,
    entities: Query<Entity, With<component::Earthbound>>,
    mut view: ResMut<GridView>,
    mut world_seeds: ResMut<model::Seeds>,
    mut world_changed: EventReader<WorldChanged>,
) {
    let got = !world_changed.is_empty();
    for event in world_changed.iter() {
        use WorldChanged::*;
        match event {
            Reset => world_seeds.reset(),
            Restart => {}
            Next => drop(world_seeds.next()),
            Last => drop(world_seeds.last()),
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
        let world = model::World::new(&seed);
        bundle::build_world(&mut commands, &world, &mapper);
        commands.insert_resource(world);
    }
}
