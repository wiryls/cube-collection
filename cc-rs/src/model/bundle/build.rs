use bevy::prelude::*;
use bevy_prototype_lyon::entity::ShapeBundle;
use bevy_prototype_lyon::prelude::*;

use super::component::*;
use crate::extra::grid::GridMapper;
use crate::model::common::Location;
use crate::model::{cube, seed};

#[derive(Bundle)]
struct CubeBundle {
    live: Earthbound,
    cube: CubeCore,
    #[bundle]
    transform: TransformBundle,
}

#[derive(Bundle)]
struct UnitBundle {
    #[bundle]
    shape: ShapeBundle,
}

pub fn spawn_cube(seed: &seed::Cube, commands: &mut Commands, mapper: &GridMapper) {
    let cube = CubeCore::from(seed);
    let zero = (cube.x(), cube.y());
    let scale = mapper.scale(1.0);
    let color = match cube.kind {
        cube::Type::White => Color::rgb(1., 1., 1.),
        cube::Type::Red => Color::rgb(1., 0., 0.),
        cube::Type::Blue => Color::rgb(0., 0., 1.),
        cube::Type::Green => Color::rgb(0., 1., 0.),
    };

    commands
        .spawn()
        .with_children(|head| {
            for (unit, pattern) in cube.body.calculate_patterns() {
                head.spawn_bundle(UnitBundle {
                    shape: GeometryBuilder::build_as(
                        &shapes::Polygon {
                            points: pattern.boundaries(1.0, 0.95),
                            closed: true,
                        },
                        DrawMode::Fill(FillMode::color(color)),
                        Transform {
                            translation: mapper.flip(unit).extend(0.),
                            ..default()
                        },
                    ),
                });
            }
        })
        .insert_bundle(CubeBundle {
            live: Earthbound::default(),
            cube,
            transform: TransformBundle {
                local: Transform {
                    translation: mapper.absolute(&zero).extend(0.),
                    scale: Vec3::new(scale, scale, 1.0),
                    ..default()
                },
                ..default()
            },
        });
}
