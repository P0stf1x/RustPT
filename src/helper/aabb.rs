use glam::Vec3;

pub mod bvh;

use crate::polygon::Vertex;

#[derive(Debug)]
pub struct AABB {
    pub min: Vec3,
    pub max: Vec3,
}

impl AABB {
    pub fn empty() -> Self {
        Self {
            min: Vec3::new(f32::INFINITY, f32::INFINITY, f32::INFINITY),
            max: Vec3::new(f32::NEG_INFINITY, f32::NEG_INFINITY, f32::NEG_INFINITY),
        }
    }

    pub fn new(vertecies: Vec<&Vertex>) -> Self {
        let mut min = Vec3::new(f32::INFINITY, f32::INFINITY, f32::INFINITY);
        let mut max = Vec3::new(f32::NEG_INFINITY, f32::NEG_INFINITY, f32::NEG_INFINITY);

        vertecies.iter().for_each(|vertex| {
            min = min.min(vertex.pos);
        });

        vertecies.iter().for_each(|vertex| {
            max = max.max(vertex.pos);
        });

        Self {
            min,
            max,
        }
    }

    pub fn expand(&self, other: &Self) -> Self {
        Self {
            min: self.min.min(other.min),
            max: self.max.max(other.max)
        }
    }

    pub fn include_vertex(&self, vertex: Vertex) -> Self {
        Self {
            min: self.min.min(vertex.pos),
            max: self.max.max(vertex.pos),
        }
    }

    pub fn center(&self) -> Vec3 {
        (self.min + self.max)/2f32
    }
}
