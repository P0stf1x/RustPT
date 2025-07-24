use glam::{Vec2, Vec3};

use crate::helper::ARGB4_to_ARGBu32;
use crate::ray::{ Ray, IntersectionResult };
use crate::screen::ScreenBuffersPixel;

#[derive(Clone, Debug)]
pub struct Vertex {
    pub pos: Vec3,
    pub uv: Vec2,
}

#[derive(Clone, Debug)]
pub struct Triangle {
    pub vertices: [Vertex; 3],
}

impl Triangle {
    pub fn intersects_ray(&self, origin: &Vec3, ray: &Ray, pixel_pointer: *mut ScreenBuffersPixel) -> Option<IntersectionResult> {
        let epsilon = f32::EPSILON;
        let edge1 = (origin+self.vertices[1].pos) - (origin+self.vertices[0].pos);
        let edge2 = (origin+self.vertices[2].pos) - (origin+self.vertices[0].pos);
        let h = ray.direction.cross(edge2);
        let a = edge1.dot(h);

        if a > -epsilon && a < epsilon {
            return None; // Ray is parallel to the triangle
        }

        let f = 1.0 / a;
        let s = ray.origin - (origin+self.vertices[0].pos);
        let u = f * s.dot(h);

        if u < 0.0 || u > 1.0 {
            return None;
        }

        let q = s.cross(edge1);
        let v = f * ray.direction.dot(q);

        if v < 0.0 || u + v > 1.0 {
            return None;
        }

        let t = f * edge2.dot(q);

        if t > epsilon {
            let intersection = IntersectionResult { distance: t, uv: ray.origin + ray.direction * t };
            if t < unsafe { *pixel_pointer }.alpha {
                unsafe { (*pixel_pointer).alpha = t };

                let w = 1.-u-v;
                let texU = (w*self.vertices[0].uv[0] + u*self.vertices[1].uv[0] + v*self.vertices[2].uv[0]).rem_euclid(255.);
                let texV = (w*self.vertices[0].uv[1] + u*self.vertices[1].uv[1] + v*self.vertices[2].uv[1]).rem_euclid(255.);

                // TODO: Proper texture rendering
                const IMG: &[u8; 786432] = include_bytes!("../../test/test_image.raw");
                let texX = (texU*512.).floor().rem_euclid(512.) as usize;
                let texY = (texV*-512.).floor().rem_euclid(512.) as usize;
                let r = IMG[texX*3 + texY*512*3];
                let g = IMG[texX*3 + texY*512*3 + 1];
                let b = IMG[texX*3 + texY*512*3 + 2];

                unsafe { (*pixel_pointer).rendered = ARGB4_to_ARGBu32(0xFF, r, g, b) };
                return Some(intersection);
            } else {
                return None;
            }
        } else {
            return None;
        }
    }

    pub fn center(&self) -> Vec3 {
        (self.vertices[0].pos + self.vertices[1].pos + self.vertices[2].pos)/3.
    }
}
