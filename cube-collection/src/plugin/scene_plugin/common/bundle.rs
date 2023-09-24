use std::time::Duration;

use bevy::prelude::*;
use bevy_prototype_lyon::entity::ShapeBundle;
use bevy_prototype_lyon::prelude::*;
use cube_core::cube::{Constraint, Kind, Movement, Neighborhood, Point};

use super::{
    super::{model::World, view::ViewMapper},
    adaption::AutoRescale,
    marker::Earthbound,
    style::{self, BoundaryBuilder},
    translate::TranslateAlpha,
};

#[derive(Bundle)]
struct DestinationBundle {
    bound: Earthbound,
    scale: AutoRescale,
    shape: ShapeBundle,
    color: Fill,
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
    shape: ShapeBundle,
    color: Fill,
}

#[derive(Bundle)]
pub struct FloorBundle {
    bound: Earthbound,
    scale: AutoRescale,
    shape: ShapeBundle,
    color: Fill,
}

pub fn build_world(commands: &mut Commands, state: &World, mapper: &ViewMapper) {
    let scale = mapper.unit();

    // draw background color
    commands.insert_resource(ClearColor(style::background_color()));

    // create destinations
    let delta = mapper.scale(&(0.5, 0.5));
    for goal in state.goals() {
        let color = style::destnation_color();
        let points = shapes::Polygon {
            points: style::cube_boundaries(Neighborhood::new(), 0.95),
            closed: true,
        };
        let translation = (mapper.locate(&goal) + delta).extend(2.);

        commands
            .spawn(DestinationBundle {
                bound: Earthbound,
                scale: AutoRescale {
                    point: goal,
                    offset: 0.5,
                },
                shape: ShapeBundle {
                    path: GeometryBuilder::build_as(&points),
                    transform: Transform {
                        translation,
                        scale: Vec3::new(scale, scale, 0.),
                        ..default()
                    },
                    ..default()
                },
                color: Fill::color(color),
            })
            .insert(TranslateAlpha::new(0.1, 0.4, Duration::from_secs(4)));
    }

    // create cubes
    let mut boundary_builder = BoundaryBuilder::new(state.width(), state.height());
    for item in state.cubes() {
        boundary_builder.put(item.position, item.neighborhood);

        let color = style::cube_color(item.kind);
        let points = shapes::Polygon {
            points: style::cube_boundaries(item.neighborhood, 0.95),
            closed: true,
        };
        let translation = (mapper.locate(&item.position) + delta).extend(1.);

        commands.spawn(CubeBundle {
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
            shape: ShapeBundle {
                path: GeometryBuilder::build_as(&points),
                transform: Transform {
                    translation,
                    scale: Vec3::new(scale, scale, 1.),
                    ..default()
                },
                ..default()
            },
            color: Fill::color(color),
        });
    }

    // create floor
    let bottom_left = Point::new(0, 0);
    let points = shapes::Polygon {
        points: boundary_builder.build(0.05),
        closed: true,
    };
    commands.spawn(FloorBundle {
        bound: Earthbound,
        scale: AutoRescale {
            point: bottom_left,
            offset: 0.,
        },
        shape: ShapeBundle {
            path: GeometryBuilder::build_as(&points),
            transform: Transform {
                translation: mapper.locate(&bottom_left).extend(0.),
                scale: Vec3::new(scale, scale, 0.),
                ..default()
            },
            ..default()
        },
        color: Fill::color(style::floor_color()),
    });
}
