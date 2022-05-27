use bevy::prelude::*;
mod plugin;
use plugin::{GridMapperUpdated, GridView};
mod rule;

mod debug;
use debug::DebugPlugin;
use rule::seed;

#[derive(Component, bevy_inspector_egui::Inspectable)]
struct Cube(i32, i32);

fn resize(
    mut query: Query<(&Cube, &mut Transform)>,
    mut relocator_updated: EventReader<GridMapperUpdated>,
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

fn setup_scene(mut commands: Commands, mut view: ResMut<GridView>) {
    let world: seed::Seed = {
        // TODO: custom an asset loader
        // https://github.com/bevyengine/bevy/discussions/3140
        let path = r"cc-rs/assets/level/tetris.level.toml";
        let data = std::fs::read_to_string(path).expect("Unable to read file");
        let s: crate::plugin::Source = toml::from_str(&data).expect("cannot parse toml");
        s.into_seed().expect("toml file is not a level")
    };

    let rect = Rect {
        left: 0,
        right: world.size.width,
        top: 0,
        bottom: world.size.height,
    };
    view.set_source(rect);

    let mapper = view.mapping();
    let scale = mapper.scale(0.9);

    for c in world.cubes {
        for o in c.body {
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

    for o in world.destnations {
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
        .add_plugin(plugin::ViewPlugin)
        .add_plugin(DebugPlugin)
        .add_startup_system(setup_scene)
        .add_system(resize)
        .run();
}
