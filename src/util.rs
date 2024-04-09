use std::{
    error::Error,
    fmt::{Display, Formatter},
};

use bevy::{math::Vec2, prelude::Vec3};
use triangulate::{TriangulationError, Vertex};

// When indexing a mesh we commonly find flat (occupying a 2 dimensional subspace) trapezes.
#[derive(Copy, Clone)]
pub(crate) struct FlatTrapezeIndices {
    pub lower_left: u32,
    pub upper_left: u32,
    pub lower_right: u32,
    pub upper_right: u32,
}

impl FlatTrapezeIndices {
    // Triangulate the trapeze
    pub fn generate_triangles(&self, indices: &mut Vec<u32>) {
        indices.push(self.upper_left);
        indices.push(self.upper_right);
        indices.push(self.lower_left);
        indices.push(self.upper_right);
        indices.push(self.lower_right);
        indices.push(self.lower_left);
    }
}

pub(crate) struct Extent {
    min: Vec3,
    max: Vec3,
}

impl Extent {
    pub fn new() -> Self {
        Extent {
            min: Vec3::new(f32::MAX, f32::MAX, f32::MAX),
            max: Vec3::new(f32::MIN, f32::MIN, f32::MIN),
        }
    }

    pub fn extend_to_include(&mut self, v: Vec3) {
        // unwrap: we know the size of this array statically
        self.min.x = f32::min(self.min.x, v.x);
        self.min.y = f32::min(self.min.y, v.y);
        self.min.z = f32::min(self.min.z, v.z);
        self.max.x = f32::max(self.max.x, v.x);
        self.max.y = f32::max(self.max.y, v.y);
        self.max.z = f32::max(self.max.z, v.z);
    }

    pub fn lengths(&self) -> Vec3 {
        self.max - self.min
    }

    pub fn center(&self) -> Vec3 {
        self.min + (self.max - self.min) / 2.0
    }
}

// This is an ugly workaround for rust's orphan rule. Neither Vec2 nor the Vertex trait come from this crate.
// So we need to implement a newtype and hope it gets optimized away (which it should).
#[derive(Debug, Copy, Clone)]
pub struct Vec2f(pub Vec2);

impl Vertex for Vec2f {
    type Coordinate = f32;

    fn x(&self) -> Self::Coordinate {
        self.0.x
    }

    fn y(&self) -> Self::Coordinate {
        self.0.y
    }
}

/// The input must not be empty.
/// No edge can cross any other edge, whether it is on the same polygon or not.
/// Each vertex must be part of exactly two edges. Polygons cannot 'share' vertices with each other.
/// Each vertex must be distinct - no vertex can have x and y coordinates that both compare equal to another vertex's.
#[derive(Debug)]
pub struct InvalidInput;

impl Display for InvalidInput {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Invalid polygon input")
    }
}

impl Error for InvalidInput {}

impl<T: Error> From<TriangulationError<T>> for InvalidInput {
    fn from(value: TriangulationError<T>) -> Self {
        match value {
            TriangulationError::TrapezoidationError(_) => {
                panic!("Failed to triangulate: {}", value)
            }
            TriangulationError::NoVertices => Self,
            TriangulationError::InternalError(_) => Self,
            TriangulationError::FanBuilder(_) => panic!("Failed to triangulate: {}", value),
            _ => panic!("Failed to triangulate: {}", value),
        }
    }
}
