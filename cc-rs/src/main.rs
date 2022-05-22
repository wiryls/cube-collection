use bevy::prelude::*;
mod view;
use view::{GridLocator, LocatorUpdated};
mod world;

mod debug;
use debug::DebugPlugin;
use world::level;

#[derive(Component, bevy_inspector_egui::Inspectable)]
struct Cube(i32, i32);

fn resize(
    mut query: Query<(&Cube, &mut Transform)>,
    mut relocator_updated: EventReader<LocatorUpdated>,
) {
    for e in relocator_updated.iter().last() {
        let value = e.mapper.scale(0.9);
        let scale = Vec3::new(value, value, 0.);
        for (cube, mut transform) in query.iter_mut() {
            transform.scale = scale;
            transform.translation = e.mapper.locate(cube.0, cube.1, 0);
        }
    }
}

fn setup_scene(mut commands: Commands, mut locator: ResMut<GridLocator>) {
    let world: level::Level = {
        // TODO: custom an asset loader
        // https://github.com/bevyengine/bevy/discussions/3140
        let path = r"cc-rs/assets/level/tetris.toml";
        let data = std::fs::read_to_string(path).expect("Unable to read file");
        let s: crate::world::toml_source::Source =
            toml::from_str(&data).expect("cannot parse toml");
        s.into_level().expect("toml file is not a level")
    };

    let size = Size::new(world.size.width, world.size.height);
    locator.set_grid(size);

    let mapper = locator.mapping();
    let scale = mapper.scale(0.9);

    for c in world.cube {
        for o in c.body {
            commands
            .spawn_bundle(SpriteBundle {
                sprite: Sprite {
                    color: match c.kind {
                        level::CubeType::White => Color::rgb(1., 1., 1.),
                        level::CubeType::Red => Color::rgb(1., 0., 0.),
                        level::CubeType::Blue => Color::rgb(0., 0., 1.),
                        level::CubeType::Green => Color::rgb(0., 1., 0.),
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

    for o in world.dest {
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
        .add_plugin(view::ViewPlugin)
        .add_plugin(DebugPlugin)
        .add_startup_system(setup_scene)
        .add_system(resize)
        .run();
}
