use glam::Vec3;

use crate::polygon::Vertex;
use crate::polygon::Triangle;
use crate::ray::{ Ray, IntersectionResult };
use crate::screen::ScreenBuffersPixel;
use crate::aabb::AABB;
// use super::*;

#[derive(Debug)]
pub struct Object {
    pub origin: Vec3,
    #[allow(dead_code)] // will be used in future
    pub rotation: Vec3,
    pub triangles: Vec<Triangle>,
    pub aabb: AABB,
}

impl Object {
    pub fn new(triangles: Vec<Triangle>) -> Self {
        let verticies: Vec<&Vertex> = triangles.iter().flat_map(|tri| {
                    tri.verticies.iter().map(|vertex| {
                        vertex
                    })
                }).collect();
        let aabb = AABB::new(verticies);
        let mut obj = Object {
            origin: Vec3::new(0., 0., 0.),
            rotation: Vec3::new(0., 0., 0.,),
            triangles,
            aabb,
        };
        return obj;
    }

    pub fn append_triangle(&mut self, triangle: Triangle) {
        self.triangles.push(triangle);
    }

    pub fn calculate_intersection(&self, ray: &Ray, pixel_pointer: *mut ScreenBuffersPixel) -> Option<IntersectionResult> {
        if !self.ray_aabb(ray) { return None; }
        let mut closest_intersection: Option<IntersectionResult> = None;
        self.triangles.iter().for_each(|triangle| {
            if let Some(current_intersection) = triangle.intersects_ray(&self.origin, &ray, pixel_pointer) {
                if closest_intersection.is_none() {
                    closest_intersection = Some(current_intersection);
                } else {
                    if current_intersection.distance < closest_intersection.unwrap().distance {
                        closest_intersection = Some(current_intersection);
                    }
                }
            }
        });
        return closest_intersection;
    }

    pub fn ray_aabb(&self, ray: &Ray) -> bool {
        // TODO: This should be additionally translated when object could be rotated

        let inv_d = Vec3::new(
            1.0 / ray.direction.x,
            1.0 / ray.direction.y,
            1.0 / ray.direction.z,
        );

        let t0 = (self.aabb.min - ray.origin + self.origin) * inv_d;
        let t1 = (self.aabb.max - ray.origin + self.origin) * inv_d;

        let tmin = t0.min(t1);
        let tmax = t0.max(t1);

        if tmin.max_element() <= tmax.min_element() && tmax.min_element() >= 0.0 {
            return true;
        } else {
            return false;
        }
    }
}
