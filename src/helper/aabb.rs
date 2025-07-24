use glam::Vec3;

pub mod bvh;

use crate::polygon::Vertex;
use crate::polygon::Triangle;

#[derive(Debug, Clone)]
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

    pub fn new(vertices: Vec<&Vertex>) -> Self {
        let mut min = Vec3::new(f32::INFINITY, f32::INFINITY, f32::INFINITY);
        let mut max = Vec3::new(f32::NEG_INFINITY, f32::NEG_INFINITY, f32::NEG_INFINITY);

        vertices.iter().for_each(|vertex| {
            min = min.min(vertex.pos);
        });

        vertices.iter().for_each(|vertex| {
            max = max.max(vertex.pos);
        });

        Self {
            min,
            max,
        }
    }

    pub fn new_from_tri(triangles: &Vec<Triangle>) -> Self {
        let vertices: Vec<&Vertex> = triangles.iter().flat_map(|tri| {
            tri.vertices.iter().map(|vertex| {
                vertex
            })
        }).collect();
        Self::new(vertices)
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
