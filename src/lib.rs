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

type Buffers = VertexBuffers<Vec2, u32>;

#[inline]
fn tessellate_fill(path: &Path, options: &FillOptions) -> Buffers {
    let mut buffers = VertexBuffers::new();
    let mut builder = BuffersBuilder::new(&mut buffers, PositionVertexConstructor);

    let mut tessellator = FillTessellator::new();
    tessellator
        .tessellate_path(path, options, &mut builder)
        .ok();

    buffers
}

#[inline]
fn construct_trimesh(buffers: Buffers) -> Collider {
    Collider::trimesh_with_flags(
        buffers.vertices,
        buffers
            .indices
            .chunks_exact(3)
            .map(|i| [i[2], i[1], i[0]])
            .collect(),
        TriMeshFlags::MERGE_DUPLICATE_VERTICES,
    )
}

pub fn stroke_trimesh(path: &Path, options: &StrokeOptions) -> Collider {
    let mut buffers = VertexBuffers::new();
    let mut builder = BuffersBuilder::new(&mut buffers, PositionVertexConstructor);

    let mut tessellator = StrokeTessellator::new();
    tessellator
        .tessellate_path(path, options, &mut builder)
        .ok();

    construct_trimesh(buffers)
}

pub fn fill_trimesh(path: &Path, options: &FillOptions) -> Collider {
    let buffers = tessellate_fill(path, options);
    construct_trimesh(buffers)
}

pub fn convex_hull(path: &Path, options: &FillOptions) -> Option<Collider> {
    let buffers = tessellate_fill(path, options);
    Collider::convex_hull(&buffers.vertices)
}
