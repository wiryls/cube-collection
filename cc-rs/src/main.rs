use bevy::prelude::*;

mod model;
use crate::model::seed;

mod extra;
use crate::extra::grid::{GridUpdated, GridView};
use crate::extra::{debug, grid, load};

#[derive(Component, bevy_inspector_egui::Inspectable)]
struct Cube(i32, i32);

fn resize(
    mut query: Query<(&Cube, &mut Transform)>,
    mut relocator_updated: EventReader<GridUpdated>,
) {
    for e in relocator_updated.iter().last() {
        let value = e.mapper.scale(0.98);
        let scale = Vec3::new(value, value, 0.);
        for (cube, mut transform) in query.iter_mut() {
            transform.scale = scale;
            transform.translation = e.mapper.locate(cube.0, cube.1, 0);
        }
    }
}

fn watch_seeds(mut status: EventReader<load::LoadSeedsUpdated>) {
    for event in status.iter() {
        use load::LoadSeedsUpdated::{Failure, Loading};
        match event {
            Loading { total, done } => println!("{}/{}", done, total),
            Failure { which } => println!("{}", which),
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
                .insert(Cube(o.x, o.y));
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
            .insert(Cube(o.x, o.y));
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(grid::GridPlugin)
        .add_plugin(load::LoaderPlugin)
        .add_plugin(debug::DebugPlugin)
        .add_system(setup_scene)
        .add_system(watch_seeds)
        .add_system(resize)
        .insert_resource(load::LoadSeeds::new(r"level/index.toml"))
        .run();
}
