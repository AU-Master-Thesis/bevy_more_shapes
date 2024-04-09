use crate::util::{FlatTrapezeIndices, InvalidInput, Vec2f};
use crate::MeshData;
use bevy::math::Vec3;
use bevy::prelude::Vec2;
use bevy::render::mesh::{Indices, Mesh};
use bevy::render::render_asset::RenderAssetUsages;
use bevy::render::render_resource::PrimitiveTopology;
use triangulate::formats::IndexedListFormat;
use triangulate::{ListFormat, TriangulationError};

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

impl TryFrom<Prism> for Mesh {
    type Error = InvalidInput;

    fn try_from(prism: Prism) -> Result<Self, Self::Error> {
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
            let indices = FlatTrapezeIndices {
                lower_left: base_index + i as u32,
                lower_right: base_index + (i + 1) as u32 % prism.points.len() as u32,
                upper_left: base_index + prism.points.len() as u32 + i as u32,
                upper_right: base_index
                    + prism.points.len() as u32
                    + (i + 1) as u32 % prism.points.len() as u32,
            };

            indices.generate_triangles(&mut mesh.indices);
        }

        // top and bottom
        let polygons = prism
            .points
            .clone()
            .into_iter()
            .map(|v| Vec2f(v))
            .collect::<Vec<Vec2f>>();
        let mut output = Vec::<[usize; 3]>::new();
        let format = IndexedListFormat::new(&mut output).into_fan_format();
        triangulate::Polygon::triangulate(&polygons, format)?;
        let indices: Vec<u32> = output
            .into_iter()
            .map(|[a, b, c]| [c, b, a])
            .flatten()
            .map(|v| v as u32)
            .collect();

        let bottom_indices = indices.clone().into_iter().rev().collect::<Vec<u32>>();
        let top_indices = indices
            .into_iter()
            .map(|i| i + (prism.points.len() as u32))
            .collect::<Vec<u32>>();
        println!("bottom_indices = {:?}", bottom_indices);
        println!("top_indices = {:?}", top_indices);
        mesh.indices.extend(bottom_indices);
        mesh.indices.extend(top_indices);

        let mut m = Mesh::new(
            PrimitiveTopology::TriangleList,
            RenderAssetUsages::MAIN_WORLD | RenderAssetUsages::RENDER_WORLD,
        );
        m.insert_attribute(Mesh::ATTRIBUTE_POSITION, mesh.positions);
        m.insert_attribute(Mesh::ATTRIBUTE_NORMAL, mesh.normals);
        m.insert_attribute(Mesh::ATTRIBUTE_UV_0, mesh.uvs);
        m.insert_indices(Indices::U32(mesh.indices));
        Ok(m)
    }
}
