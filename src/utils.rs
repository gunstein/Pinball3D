use bevy::prelude::*;
use bevy::{
    prelude::Mesh,
    render::{
        mesh::{Indices, VertexAttributeValues},
    },
};
use bevy_rapier3d::prelude::*;


pub fn mesh_to_convexdecomp_collider_shape(mesh: &Mesh) -> Option<Collider> {
    info!("test1");
    let vertices = if let Some(VertexAttributeValues::Float32x3(positions)) =
        mesh.attribute(Mesh::ATTRIBUTE_POSITION)
    {
        positions
            .iter()
            .map(|p| Vec3::from_slice(p))
            .collect::<Vec<_>>()
    } else {
        return None;
    };
    let indices = if let Some(Indices::U32(indices)) = mesh.indices() {
        indices
            .chunks_exact(3)
            .map(|tri| [tri[0], tri[1], tri[2]])
            .collect::<Vec<_>>()
    } else {
        return None;
    };
    info!("test2");
    Some(Collider::convex_decomposition(&vertices, &indices))
}
