use bevy::prelude::Vec2;
use bevy_rapier2d::prelude::{Collider, TriMeshFlags};
use lyon_tessellation::{
    path::Path, BuffersBuilder, FillOptions, FillTessellator, FillVertex, FillVertexConstructor,
    StrokeOptions, StrokeTessellator, StrokeVertex, StrokeVertexConstructor, VertexBuffers,
};

struct PositionVertexConstructor;
impl FillVertexConstructor<Vec2> for PositionVertexConstructor {
    fn new_vertex(&mut self, vertex: FillVertex) -> Vec2 {
        let position = vertex.position();
        Vec2::new(position.x, position.y)
    }
}
impl StrokeVertexConstructor<Vec2> for PositionVertexConstructor {
    fn new_vertex(&mut self, vertex: StrokeVertex) -> Vec2 {
        let position = vertex.position();
        Vec2::new(position.x, position.y)
    }
}

#[derive(Debug, Default, Clone, Copy)]
pub enum ColliderType {
    #[default]
    ConvexHull,
    Trimesh,
}

#[derive(Debug, Clone, Copy)]
pub enum TessellationType {
    Fill(FillOptions),
    Stroke(StrokeOptions),
}

impl Default for TessellationType {
    fn default() -> Self {
        TessellationType::Fill(FillOptions::default())
    }
}

#[derive(Debug, Default, Clone, Copy)]
pub struct ColliderConstructor {
    pub tessellation_type: TessellationType,
    pub collider_type: ColliderType,
}

impl ColliderConstructor {
    pub fn construct(&self, path: &Path) -> Option<Collider> {
        let mut vertex_buffers: VertexBuffers<Vec2, u32> = VertexBuffers::new();
        let mut vertex_builder =
            BuffersBuilder::new(&mut vertex_buffers, PositionVertexConstructor);

        match self.tessellation_type {
            TessellationType::Fill(options) => {
                let mut tessellator = FillTessellator::new();
                tessellator
                    .tessellate_path(path, &options, &mut vertex_builder)
                    .ok();
            }
            TessellationType::Stroke(options) => {
                let mut tessellator = StrokeTessellator::new();
                tessellator
                    .tessellate_path(path, &options, &mut vertex_builder)
                    .ok();
            }
        };

        match self.collider_type {
            ColliderType::ConvexHull => Collider::convex_hull(&vertex_buffers.vertices),
            ColliderType::Trimesh => Some(Collider::trimesh_with_flags(
                vertex_buffers.vertices,
                vertex_buffers
                    .indices
                    .chunks_exact(3)
                    .map(|i| [i[0], i[1], i[2]])
                    .collect(),
                TriMeshFlags::MERGE_DUPLICATE_VERTICES,
            )),
        }
    }
}
