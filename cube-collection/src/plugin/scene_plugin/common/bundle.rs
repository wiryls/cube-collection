use std::time::Duration;

use bevy::prelude::*;
use bevy_prototype_lyon::entity::Shape;
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
    shape: Shape,
    transform: Transform,
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
    shape: Shape,
    transform: Transform,
}

#[derive(Bundle)]
pub struct FloorBundle {
    bound: Earthbound,
    scale: AutoRescale,
    shape: Shape,
    transform: Transform,
}

pub fn hello_world(commands: &mut Commands, state: &World, mapper: &ViewMapper) {
    fn make_polygon(points: Vec<Vec2>, color: Color) -> Shape {
        ShapeBuilder::with(&shapes::Polygon {
            points,
            closed: true,
        })
        .fill(color)
        .build()
    }

    let scale = mapper.unit();

    // draw background color
    commands.insert_resource(ClearColor(style::background_color()));

    // create destinations
    let delta = mapper.scale(&(0.5, 0.5));
    for goal in state.goals() {
        commands
            .spawn(DestinationBundle {
                bound: Earthbound,
                scale: AutoRescale {
                    point: goal,
                    offset: 0.5,
                },
                shape: make_polygon(
                    style::cube_boundaries(Neighborhood::new(), 0.95),
                    style::destination_color(),
                ),
                transform: Transform {
                    translation: (mapper.locate(&goal) + delta).extend(2.),
                    scale: Vec3::new(scale, scale, 0.),
                    ..default()
                },
            })
            .insert(TranslateAlpha::new(0.1, 0.4, Duration::from_secs(4)));
    }

    // create cubes
    let mut boundary_builder = BoundaryBuilder::new(state.width(), state.height());
    for item in state.cubes() {
        boundary_builder.put(item.position, item.neighborhood);
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
            shape: make_polygon(
                style::cube_boundaries(item.neighborhood, 0.95),
                style::cube_color(item.kind),
            ),
            transform: Transform {
                translation: (mapper.locate(&item.position) + delta).extend(1.),
                scale: Vec3::new(scale, scale, 1.),
                ..default()
            },
        });
    }

    // create floor
    let bottom_left = Point::new(0, 0);
    commands.spawn(FloorBundle {
        bound: Earthbound,
        scale: AutoRescale {
            point: bottom_left,
            offset: 0.,
        },
        shape: make_polygon(boundary_builder.build(0.05), style::floor_color()),
        transform: Transform {
            translation: mapper.locate(&bottom_left).extend(0.),
            scale: Vec3::new(scale, scale, 0.),
            ..default()
        },
    });
}
