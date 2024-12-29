use glam::Vec3;

use crate::polygon::Triangle;
use crate::ray::{ Ray, IntersectionResult };
use crate::screen::ScreenBuffersPixel;
// use super::*;

#[derive(Debug)]
pub struct Object {
    pub origin: Vec3,
    #[allow(dead_code)] // will be used in future
    pub rotation: Vec3,
    pub triangles: Vec<Triangle>,
}

impl Object {
    pub fn append_triangle(&mut self, triangle: Triangle) {
        self.triangles.push(triangle);
    }

    pub fn calculate_intersection(&self, ray: &Ray, pixel_pointer: *mut ScreenBuffersPixel) -> Option<IntersectionResult> {
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
}
