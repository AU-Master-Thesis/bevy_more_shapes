use crate::util::FlatTrapezeIndices;
use crate::MeshData;
use bevy::math::Vec3;
use bevy::prelude::Vec2;
use bevy::render::mesh::{Indices, Mesh};
use bevy::render::render_asset::RenderAssetUsages;
use bevy::render::render_resource::PrimitiveTopology;

pub struct Prism {
    pub height: f32,
    pub points: Vec<Vec2>,
}

impl Prism {
    pub fn new(height: f32, points: Vec<Vec2>) -> Self {
        assert!(height > 0.0, "Height must be positive");
        assert!(
            points.len() >= 3,
            "At least 3 points are required to form a prism"
        );
        Self { height, points }
    }
}

impl From<Prism> for Mesh {
    // type Error = TriangulationError;

    fn from(prism: Prism) -> Self {
        let mut mesh = MeshData::new(prism.points.len() * 2, prism.points.len() * 6);

        let base_index = mesh.positions.len() as u32;

        // Bottom
        for point in &prism.points {
            mesh.positions.push(Vec3::new(point.x, 0.0, point.y));
            mesh.normals.push(-Vec3::Y);
            mesh.uvs.push(Vec2::new(point.x, point.y));
        }

        // Top
        for point in &prism.points {
            mesh.positions
                .push(Vec3::new(point.x, prism.height, point.y));
            mesh.normals.push(Vec3::Y);
            mesh.uvs.push(Vec2::new(point.x, point.y));
        }

        // Indices
        for i in 0..prism.points.len() {
            let next_i = (i + 1) % prism.points.len();
            let bottom_i = base_index + i as u32;
            let top_i = base_index + prism.points.len() as u32 + i as u32;

            mesh.indices.push(bottom_i);
            mesh.indices.push(top_i);
            mesh.indices.push(base_index + next_i as u32);

            mesh.indices.push(base_index + next_i as u32);
            mesh.indices.push(top_i);
            mesh.indices
                .push(base_index + prism.points.len() as u32 + next_i as u32);
        }

        let mut m = Mesh::new(
            PrimitiveTopology::TriangleList,
            RenderAssetUsages::MAIN_WORLD | RenderAssetUsages::RENDER_WORLD,
        );
        m.insert_attribute(Mesh::ATTRIBUTE_POSITION, mesh.positions);
        m.insert_attribute(Mesh::ATTRIBUTE_NORMAL, mesh.normals);
        m.insert_attribute(Mesh::ATTRIBUTE_UV_0, mesh.uvs);
        m.insert_indices(Indices::U32(mesh.indices));
        m
    }
}
