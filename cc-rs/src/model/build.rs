use bevy::prelude::*;
use bevy_prototype_lyon::entity::ShapeBundle;
use bevy_prototype_lyon::prelude::*;

use super::cube::*;
use super::{detail, seed};
use crate::extra::grid::GridMapper;

#[derive(Bundle)]
struct CubeBundle {
    live: Live,
    kind: Type,
    pack: Pack,
    #[bundle]
    transform: TransformBundle,
}

#[derive(Bundle)]
struct UnitBundle {
    live: Live,
    point: GridPoint,
    #[bundle]
    shape: ShapeBundle,
}

pub fn spawn_cube(cube: &seed::Cube, commands: &mut Commands, mapper: &GridMapper) {
    let pack = Pack::from(detail::United::from(cube.body.iter()));
    let zero = (pack.0.rect.left, pack.0.rect.top);
    let scale = mapper.scale();
    let color = match cube.kind {
        Type::White => Color::rgb(1., 1., 1.),
        Type::Red => Color::rgb(1., 0., 0.),
        Type::Blue => Color::rgb(0., 0., 1.),
        Type::Green => Color::rgb(0., 1., 0.),
    };

    commands
        .spawn()
        .with_children(|head| {
            for unit in &pack.0.units {
                head.spawn_bundle(UnitBundle {
                    live: Live {},
                    point: GridPoint::from(&unit.o),
                    shape: GeometryBuilder::build_as(
                        &shapes::Polygon {
                            points: unit.v.boundaries(1.0, 0.95),
                            closed: true,
                        },
                        DrawMode::Fill(FillMode::color(color)),
                        Transform {
                            translation: mapper.relative(unit).extend(0.),
                            scale: Vec3::new(scale, scale, 1.0),
                            ..default()
                        },
                    ),
                });
            }
        })
        .insert_bundle(CubeBundle {
            live: Live {},
            kind: cube.kind,
            pack,
            transform: TransformBundle {
                local: Transform {
                    translation: mapper.absolute(&zero).extend(0.),
                    ..default()
                },
                ..default()
            },
        });
}
