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
    style: Pattern,
    point: GridPoint,
    #[bundle]
    shape: ShapeBundle,
}

pub struct CubeBuilder<'a>(&'a seed::Cube);

impl detail::Location<i32> for seed::Location {
    fn x_(&self) -> i32 {
        self.x
    }

    fn y_(&self) -> i32 {
        self.y
    }
}

impl<'a> CubeBuilder<'a> {
    pub fn new(cube: &'a seed::Cube) -> Self {
        Self(cube)
    }

    pub fn build(&self, commands: &mut Commands, mapper: &GridMapper) {
        let pack = Pack::from(detail::United::from(self.0.body.iter()));
        let zero = (pack.0.rect.left, pack.0.rect.top);
        let scale = mapper.unit();
        let color = match self.0.kind {
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
                        style: Pattern::from(&unit.n),
                        point: GridPoint {
                            x: unit.o.x,
                            y: unit.o.y,
                        },
                        shape: GeometryBuilder::build_as(
                            &shapes::Polygon {
                                points: detail::make_boundaries(scale, 0.95, unit.n),
                                closed: true,
                            },
                            DrawMode::Fill(FillMode::color(color)),
                            Transform {
                                translation: mapper.scale(unit.o.x, unit.o.y, 0.),
                                ..default()
                            },
                        ),
                    });
                }
            })
            .insert_bundle(CubeBundle {
                live: Live {},
                kind: self.0.kind,
                pack,
                transform: TransformBundle {
                    local: Transform {
                        translation: mapper.locate(zero.0, zero.1, 0.),
                        ..default()
                    },
                    ..default()
                },
            });
    }
}
