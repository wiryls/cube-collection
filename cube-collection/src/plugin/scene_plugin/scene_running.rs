use bevy::ecs::schedule::BaseSystemSet;
use bevy::prelude::*;

use super::{
    common::{bundle, component, system},
    model,
    view::{GridView, ViewRect, ViewUpdated},
    SceneState,
};

pub fn setup(
    app: &mut App,
    first: impl BaseSystemSet,
    second: impl BaseSystemSet,
    third: impl BaseSystemSet,
) {
    app.add_event::<WorldChanged>()
        .add_system(setup_world.in_schedule(OnEnter(SceneState::Running)))
        .add_systems(
            (
                system::position.run_if(resource_exists::<model::World>()),
                system::realpha.run_if(resource_exists::<model::World>()),
                system::recolor.run_if(resource_exists::<model::World>()),
                system::reshape.run_if(resource_exists::<model::World>()),
            )
                .in_base_set(first),
        )
        .add_systems((system::state.run_if(resource_exists::<model::World>()),).in_base_set(second))
        .add_systems(
            (
                switch_world.run_if(on_event::<WorldChanged>()),
                system::self_adaption.run_if(on_event::<ViewUpdated>()),
            )
                .in_base_set(third),
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
        view.set_source(ViewRect {
            top: 0,
            bottom: seed.size.height,
            left: 0,
            right: seed.size.width,
        });
        let mapper = view.mapping();

        // [2] create new world
        let world = model::World::new(&seed);
        bundle::build_world(&mut commands, &world, &mapper);
        commands.insert_resource(world);
    }
}
