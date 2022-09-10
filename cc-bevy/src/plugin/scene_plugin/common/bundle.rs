use std::time::Duration;

use bevy::prelude::*;
use bevy_prototype_lyon::entity::ShapeBundle;
use bevy_prototype_lyon::prelude::*;
use cc_core::cube::{Constraint, Kind, Movement, Neighborhood, Point};

use super::{
    super::{model::World, view::ViewMapper},
    adaption::AutoRescale,
    marker::Earthbound,
    style,
    translate::TranslateAlpha,
};

#[derive(Bundle)]
struct DestinationBundle {
    bound: Earthbound,
    scale: AutoRescale,
    #[bundle]
    shape: ShapeBundle,
}

#[derive(Component)]
pub struct Cubic {
    pub id: usize,
    pub kind: Kind,
    pub movement: Option<Movement>,
    pub constraint: Constraint,
    pub neighborhood: Neighborhood,
}

#[derive(Bundle)]
struct CubeBundle {
    cubic: Cubic,
    bound: Earthbound,
    scale: AutoRescale,
    #[bundle]
    shape: ShapeBundle,
}

#[derive(Bundle)]
pub struct FloorBundle {
    bound: Earthbound,
    scale: AutoRescale,
    #[bundle]
    shape: ShapeBundle,
}

pub fn build_world(commands: &mut Commands, state: &World, mapper: &ViewMapper) {
    let scale = mapper.unit();

    // background color
    commands.insert_resource(ClearColor(style::background_color()));

    // create floor
    let right = state.width();
    let bottom = state.height();
    commands.spawn_bundle(FloorBundle {
        bound: Earthbound,
        scale: AutoRescale {
            point: Point::default(),
            offset: 0.,
        },
        shape: GeometryBuilder::build_as(
            &shapes::Polygon {
                points: [(0, 0), (0, bottom), (right, bottom), (right, 0)]
                    .iter()
                    .map(|o| mapper.flip(o))
                    .collect(),
                closed: true,
            },
            DrawMode::Fill(FillMode::color(style::floor_color())),
            Transform {
                translation: mapper.locate(&(0i32, 0)).extend(0.),
                scale: Vec3::new(scale, scale, 0.),
                ..default()
            },
        ),
    });

    // create destnations
    let delta = mapper.scale(&(0.5, 0.5));
    for goal in state.goals() {
        let color = style::destnation_color();
        let points = style::cube_boundaries(Neighborhood::new(), 1., 0.95);
        let translation = (mapper.locate(&goal) + delta).extend(2.);

        commands
            .spawn_bundle(DestinationBundle {
                bound: Earthbound,
                scale: AutoRescale {
                    point: goal,
                    offset: 0.5,
                },
                shape: GeometryBuilder::build_as(
                    &shapes::Polygon {
                        points,
                        closed: true,
                    },
                    DrawMode::Fill(FillMode::color(color)),
                    Transform {
                        translation,
                        scale: Vec3::new(scale, scale, 0.),
                        ..default()
                    },
                ),
            })
            .insert(TranslateAlpha::new(0.1, 0.3, Duration::from_secs(6)));
    }

    // create cubes
    for item in state.cubes() {
        let color = style::cube_color(item.kind);
        let points = style::cube_boundaries(item.neighborhood, 1., 0.95);
        let translation = (mapper.locate(&item.position) + delta).extend(1.);

        commands.spawn_bundle(CubeBundle {
            cubic: Cubic {
                id: item.id,
                kind: item.kind,
                movement: None,
                constraint: Constraint::Free,
                neighborhood: item.neighborhood,
            },
            bound: Earthbound,
            scale: AutoRescale {
                point: item.position,
                offset: 0.5,
            },
            shape: GeometryBuilder::build_as(
                &shapes::Polygon {
                    points,
                    closed: true,
                },
                DrawMode::Fill(FillMode::color(color)),
                Transform {
                    translation,
                    scale: Vec3::new(scale, scale, 1.),
                    ..default()
                },
            ),
        });
    }
}
