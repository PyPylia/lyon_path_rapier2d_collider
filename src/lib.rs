use bevy::prelude::Vec2;
use lyon_tessellation::{
    path::Path, BuffersBuilder, FillOptions, FillTessellator, FillVertex, FillVertexConstructor,
    StrokeOptions, StrokeTessellator, StrokeVertex, StrokeVertexConstructor, VertexBuffers,
};
use bevy_rapier2d::prelude::{Collider, TriMeshFlags};

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

pub enum ColliderConstructor {
    Fill(FillOptions),
    Stroke(StrokeOptions),
}

impl ColliderConstructor {
    pub fn default_fill() -> Self {
        ColliderConstructor::Fill(FillOptions::default())
    }

    pub fn default_stroke(line_width: f32) -> Self {
        ColliderConstructor::Stroke(StrokeOptions::default().with_line_width(line_width))
    }

    pub fn construct(&self, path: &Path) -> Collider {
        let mut vertex_buffers: VertexBuffers<Vec2, u32> = VertexBuffers::new();
        let mut vertex_builder =
            BuffersBuilder::new(&mut vertex_buffers, PositionVertexConstructor);

        match self {
            ColliderConstructor::Fill(options) => {
                let mut tessellator = FillTessellator::new();
                tessellator
                    .tessellate_path(path, options, &mut vertex_builder)
                    .ok();
            }
            ColliderConstructor::Stroke(options) => {
                let mut tessellator = StrokeTessellator::new();
                tessellator
                    .tessellate_path(path, options, &mut vertex_builder)
                    .ok();
            }
        };

        Collider::trimesh_with_flags(
            vertex_buffers.vertices,
            vertex_buffers
                .indices
                .chunks_exact(3)
                .map(|i| [i[0], i[1], i[2]])
                .collect(),
            TriMeshFlags::MERGE_DUPLICATE_VERTICES,
        )
    }
}
