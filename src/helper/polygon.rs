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
    pub verticies: [Vertex; 3],
}

impl Triangle {
    pub fn intersects_ray(&self, origin: &Vec3, ray: &Ray, pixel_pointer: *mut ScreenBuffersPixel) -> Option<IntersectionResult> {
        let epsilon = f32::EPSILON;
        let edge1 = (origin+self.verticies[1].pos) - (origin+self.verticies[0].pos);
        let edge2 = (origin+self.verticies[2].pos) - (origin+self.verticies[0].pos);
        let h = ray.direction.cross(edge2);
        let a = edge1.dot(h);

        if a > -epsilon && a < epsilon {
            return None; // Ray is parallel to the triangle
        }

        let f = 1.0 / a;
        let s = ray.origin - (origin+self.verticies[0].pos);
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
                let multiplier = 1000.0;

                let w = 1.-u-v;
                let texU = (w*self.verticies[0].uv[0] + u*self.verticies[1].uv[0] + v*self.verticies[2].uv[0]).rem_euclid(255.);
                let texV = (w*self.verticies[0].uv[1] + u*self.verticies[1].uv[1] + v*self.verticies[2].uv[1]).rem_euclid(255.);
                let r = f32::clamp(texU*255., 0.0, 255.0) as u8;
                let g = f32::clamp(texV*255., 0.0, 255.0) as u8;
                let b = f32::clamp(255., 0.0, 255.0) as u8;
                // let r = f32::clamp(intersection.uv.x*multiplier, 0.0, 255.0) as u8;
                // let g = f32::clamp(intersection.uv.y*multiplier, 0.0, 255.0) as u8;
                // let b = f32::clamp(intersection.uv.z*multiplier, 0.0, 255.0) as u8;
                // unsafe { (*pixel_pointer).rendered = ARGB4_to_ARGBu32(0xFF, 0xFF, 0xFF, 0xFF) };
                unsafe { (*pixel_pointer).rendered = ARGB4_to_ARGBu32(0xFF, r, g, b) };
                return Some(intersection);
            } else {
                return None;
            }
        } else {
            return None;
        }
    }
}
