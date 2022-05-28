use bevy::prelude::*;
use iyes_loopless::prelude::*;

use super::state::State;
use crate::extra::grid::{GridUpdated, GridView};
use crate::model::cube;
use crate::model::seed;

pub struct RunningScene;
impl Plugin for RunningScene {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            ConditionSet::new()
                .run_in_state(State::Running)
                .with_system(regrid)
                .with_system(setup_scene)
                .into(),
        );
    }
}

fn regrid(
    mut query: Query<(&cube::Gridded, &mut Transform)>,
    mut relocator_updated: EventReader<GridUpdated>,
) {
    for e in relocator_updated.iter().last() {
        let value = e.mapper.scale(0.98);
        let scale = Vec3::new(value, value, 0.);
        for (cube, mut transform) in query.iter_mut() {
            transform.scale = scale;
            transform.translation = e.mapper.locate(cube.x, cube.y, 0);
        }
    }
}

fn setup_scene(
    mut commands: Commands,
    mut view: ResMut<GridView>,
    seeds: Option<Res<seed::Seeds>>,
) {
    let seeds = match seeds {
        None => return,
        Some(seeds) => {
            commands.remove_resource::<seed::Seeds>();
            seeds
        }
    };

    let world = seeds.first().unwrap();
    let rect = Rect {
        left: 0,
        right: world.size.width,
        top: 0,
        bottom: world.size.height,
    };
    view.set_source(rect);

    let mapper = view.mapping();
    let scale = mapper.scale(0.98);

    for c in &world.cubes {
        for o in &c.body {
            commands
                .spawn_bundle(SpriteBundle {
                    sprite: Sprite {
                        color: match c.kind {
                            seed::CubeType::White => Color::rgb(1., 1., 1.),
                            seed::CubeType::Red => Color::rgb(1., 0., 0.),
                            seed::CubeType::Blue => Color::rgb(0., 0., 1.),
                            seed::CubeType::Green => Color::rgb(0., 1., 0.),
                        },
                        ..default()
                    },
                    transform: Transform {
                        scale: Vec3::new(scale, scale, 0.),
                        translation: mapper.locate(o.x, o.y, 0.),
                        ..default()
                    },
                    ..default()
                })
                .insert(cube::Gridded { x: o.x, y: o.y });
        }
    }

    for o in &world.destnations {
        commands
            .spawn_bundle(SpriteBundle {
                sprite: Sprite {
                    color: Color::rgb(0.1, 0.1, 0.1),
                    ..default()
                },
                transform: Transform {
                    scale: Vec3::new(scale, scale, 0.),
                    translation: mapper.locate(o.x, o.y, 0.),
                    ..default()
                },
                ..default()
            })
            .insert(cube::Gridded { x: o.x, y: o.y });
    }
}
