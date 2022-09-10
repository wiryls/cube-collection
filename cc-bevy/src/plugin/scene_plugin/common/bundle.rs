use std::time::Duration;

use bevy::prelude::*;
use bevy_prototype_lyon::entity::ShapeBundle;
use bevy_prototype_lyon::prelude::*;
use cc_core::cube::{Constraint, Kind, Movement, Neighborhood};

use super::{
    super::{model::World, view::ViewMapper},
    marker::Earthbound,
    positioned::Gridded,
    style,
    translate::TranslateAlpha,
};

#[derive(Bundle)]
struct DestinationBundle {
    point: Gridded,
    bound: Earthbound,
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
    point: Gridded,
    bound: Earthbound,
    #[bundle]
    shape: ShapeBundle,
}

pub fn spawn_objects(commands: &mut Commands, state: &World, mapper: &ViewMapper) {
    let scale = mapper.scale(1.0f32);

    for goal in state.goals() {
        let color = Color::rgb(0.3, 0.3, 0.3);
        let points = style::cube_boundaries(Neighborhood::new(), 1., 0.95);
        let translation = mapper.absolute(&goal).extend(0.);

        commands
            .spawn_bundle(DestinationBundle {
                point: goal.into(),
                bound: Earthbound::default(),
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
            .insert(TranslateAlpha::new(0.1, 0.3, Duration::from_secs(8)));
    }

    for item in state.cubes() {
        let color = style::cube_color(item.kind);
        let points = style::cube_boundaries(item.neighborhood, 1., 0.95);
        let translation = mapper.absolute(&item.position).extend(1.);

        commands.spawn_bundle(CubeBundle {
            cubic: Cubic {
                id: item.id,
                kind: item.kind,
                movement: None,
                constraint: Constraint::Free,
                neighborhood: item.neighborhood,
            },
            point: item.position.into(),
            bound: Earthbound::default(),
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
