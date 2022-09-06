use bevy::prelude::*;
use bevy_prototype_lyon::entity::ShapeBundle;
use bevy_prototype_lyon::prelude::*;
use cc_core::{
    cube::{Constraint, Kind, Movement, Neighborhood, Point},
    Unit,
};

use super::{super::view::ViewMapper, style, world::World};

#[derive(Component, Default)]
pub struct Earthbound;

#[derive(Component)]
pub struct Cubic {
    pub id: usize,
    pub kind: Kind,
    pub position: Point,
    pub movement: Option<Movement>,
    pub constraint: Constraint,
    pub neighborhood: Neighborhood,
}

impl From<Unit> for Cubic {
    fn from(item: Unit) -> Self {
        Self {
            id: item.id,
            kind: item.kind,
            position: item.position,
            movement: None,
            constraint: Constraint::Free,
            neighborhood: item.neighborhood,
        }
    }
}

#[derive(Bundle)]
struct CubeBundle {
    cubic: Cubic,
    bound: Earthbound,
    #[bundle]
    shape: ShapeBundle,
}

pub fn spawn_cubes(state: &World, commands: &mut Commands, mapper: &ViewMapper) {
    let scale = mapper.scale(1.0f32);
    for item in state.cubes() {
        let color = style::cube_color(item.kind);
        let points = style::cube_boundaries(item.neighborhood, 1., 0.95);
        let translation = mapper.absolute(&item.position).extend(0.);

        commands.spawn_bundle(CubeBundle {
            cubic: item.into(),
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
