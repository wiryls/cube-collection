use bevy::{
    asset::{uuid_handle, RenderAssetUsages},
    color::ColorToComponents,
    mesh::Indices,
    prelude::*,
    render::render_resource::PrimitiveTopology,
};
use lyon_tessellation::{
    math::point, path::Path, BuffersBuilder, FillOptions, FillTessellator, FillVertex,
    FillVertexConstructor, VertexBuffers,
};

/////////////////////////////////////////////////////////////////////////////
// polygon

const COLOR_MATERIAL_HANDLE: Handle<ColorMaterial> =
    uuid_handle!("5d5a2c72-d28e-488a-80b3-7dd29de5bd14");

#[derive(Component, Clone)]
#[require(Mesh2d, MeshMaterial2d<ColorMaterial> = color_material_handle(), Transform, Visibility)]
pub struct Polygon {
    path: Path,
    color: Color,
}

impl Polygon {
    pub fn new(points: Vec<Vec2>, color: Color) -> Self {
        Self {
            path: build_path(points),
            color,
        }
    }

    pub fn set_points(&mut self, points: Vec<Vec2>) {
        self.path = build_path(points);
    }

    pub fn set_color(&mut self, color: Color) {
        self.color = color;
    }

    pub fn set_alpha(&mut self, alpha: f32) {
        self.color.set_alpha(alpha);
    }
}

fn color_material_handle() -> MeshMaterial2d<ColorMaterial> {
    MeshMaterial2d(COLOR_MATERIAL_HANDLE)
}

fn build_path(points: Vec<Vec2>) -> Path {
    let mut path = Path::builder();
    let mut points = points.into_iter();
    if let Some(first) = points.next() {
        path.begin(point(first.x, first.y));
        for vertex in points {
            path.line_to(point(vertex.x, vertex.y));
        }
        path.end(true);
    }
    path.build()
}

/////////////////////////////////////////////////////////////////////////////
// plugin

pub fn setup(app: &mut App) {
    app.init_resource::<PolygonTessellator>().add_systems(
        PostUpdate,
        mesh_polygons_system
            .after(bevy::transform::TransformSystems::Propagate)
            .before(bevy::asset::AssetEventSystems),
    );

    if let Some(mut materials) = app.world_mut().get_resource_mut::<Assets<ColorMaterial>>() {
        // Inserting an asset with a UUID handle cannot fail.
        let _ = materials.insert(&COLOR_MATERIAL_HANDLE, ColorMaterial::default());
    } else {
        error!("Failed to get Assets<ColorMaterial> resource");
    }
}

/////////////////////////////////////////////////////////////////////////////
// mesh

fn mesh_polygons_system(
    mut meshes: ResMut<Assets<Mesh>>,
    mut tessellator: ResMut<PolygonTessellator>,
    mut query: Query<(&Polygon, &mut Mesh2d), Changed<Polygon>>,
) {
    for (polygon, mut mesh) in &mut query {
        mesh.0 = meshes.add(build_mesh(&mut tessellator, polygon));
    }
}

fn build_mesh(tessellator: &mut FillTessellator, polygon: &Polygon) -> Mesh {
    let mut buffers: VertexBuffers<Vertex, u32> = VertexBuffers::new();
    if let Err(error) = tessellator.tessellate_path(
        &polygon.path,
        &FillOptions::default(),
        &mut BuffersBuilder::new(
            &mut buffers,
            VertexConstructor {
                color: polygon.color,
            },
        ),
    ) {
        error!(?error, "Failed to tessellate polygon");
    }

    let mut mesh = Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::default(),
    );
    mesh.insert_indices(Indices::U32(buffers.indices));
    mesh.insert_attribute(
        Mesh::ATTRIBUTE_POSITION,
        buffers
            .vertices
            .iter()
            .map(|vertex| [vertex.position.x, vertex.position.y, 0.0])
            .collect::<Vec<_>>(),
    );
    mesh.insert_attribute(
        Mesh::ATTRIBUTE_COLOR,
        buffers
            .vertices
            .iter()
            .map(|vertex| vertex.color)
            .collect::<Vec<_>>(),
    );
    mesh
}

#[derive(Default, Deref, DerefMut, Resource)]
struct PolygonTessellator(FillTessellator);

#[derive(Clone, Copy)]
struct Vertex {
    position: Vec2,
    color: [f32; 4],
}

struct VertexConstructor {
    color: Color,
}

impl FillVertexConstructor<Vertex> for VertexConstructor {
    fn new_vertex(&mut self, vertex: FillVertex) -> Vertex {
        Vertex {
            position: Vec2::new(vertex.position().x, vertex.position().y),
            color: self.color.to_linear().to_f32_array(),
        }
    }
}

/////////////////////////////////////////////////////////////////////////////
// tests

#[cfg(test)]
mod tests {
    use super::*;
    use crate::plugin::scene_plugin::common::style;
    use cube_core::cube::Neighborhood;

    #[test]
    fn tessellates_concave_polygon() {
        let polygon = Polygon::new(
            vec![
                Vec2::new(0.0, 0.0),
                Vec2::new(2.0, 0.0),
                Vec2::new(2.0, 2.0),
                Vec2::new(1.0, 1.0),
                Vec2::new(0.0, 2.0),
            ],
            Color::WHITE,
        );
        let mesh = build_mesh(&mut FillTessellator::new(), &polygon);

        assert_eq!(mesh.primitive_topology(), PrimitiveTopology::TriangleList);
        assert!(mesh.count_vertices() >= 5);
        assert!(mesh.indices().is_some_and(|indices| indices.len() >= 9));
        assert!(mesh.attribute(Mesh::ATTRIBUTE_COLOR).is_some());
    }

    #[test]
    fn tessellates_every_cube_neighborhood() {
        let mut tessellator = FillTessellator::new();
        for bits in 0..=u8::MAX {
            let neighborhood =
                Neighborhood::from(Neighborhood::AROUNDS.into_iter().enumerate().filter_map(
                    |(index, adjacency)| (bits & (1 << index) != 0).then_some(adjacency),
                ));
            let polygon = Polygon::new(style::cube_boundaries(neighborhood, 0.95), Color::WHITE);
            let mesh = build_mesh(&mut tessellator, &polygon);

            assert!(
                mesh.count_vertices() >= 4,
                "neighborhood bits: {bits:#010b}"
            );
            assert!(
                mesh.indices().is_some_and(|indices| indices.len() >= 6),
                "neighborhood bits: {bits:#010b}"
            );
        }
    }
}
