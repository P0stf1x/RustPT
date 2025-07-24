use std::sync::Arc;
use std::sync::RwLock;

use glam::Vec3;

use crate::polygon::Vertex;
use crate::polygon::Triangle;
use crate::ray::{ Ray, IntersectionResult };
use crate::screen::ScreenBuffersPixel;
use crate::aabb::AABB;
use crate::aabb::bvh::*;
// use super::*;

#[derive(Debug)]
pub struct Object {
    pub origin: Vec3,
    #[allow(dead_code)] // will be used in future
    pub rotation: Vec3,
    pub triangles: Vec<Arc<RwLock<Triangle>>>,
    pub aabb: AABB,
    pub bvh: BVH,
}

impl Object {
    pub fn new(triangles: Vec<Triangle>) -> Self {
        let vertices: Vec<&Vertex> = triangles.iter().flat_map(|tri| {
            tri.vertices.iter().map(|vertex| {
                vertex
            })
        }).collect();
        let tris: Vec<Arc<RwLock<Triangle>>> = triangles.iter().map(|tri| {
            Arc::new(RwLock::new(tri.clone()))
        }).collect();
        let aabb = AABB::new_from_tri(&triangles); 
        let bvh = BVH::generate_bottom(&tris);
        let obj = Object {
            origin: Vec3::new(0., 0., 0.),
            rotation: Vec3::new(0., 0., 0.,),
            triangles: tris,
            aabb,
            bvh,
        };
        return obj;
    }

    pub fn append_triangle(&mut self, triangle: Triangle) {
        self.triangles.push(Arc::new(RwLock::new(triangle)));
    }

    pub fn append_triangle_array(&mut self, triangles: &[Triangle]) {
        triangles.iter().for_each(|tri| {
            self.append_triangle(tri.clone());
        });
    }

    pub fn calculate_intersection(&self, ray: &Ray, pixel_pointer: *mut ScreenBuffersPixel) -> Option<IntersectionResult> {
        if !self.ray_aabb(ray) { return None; }
        let mut closest_intersection: Option<IntersectionResult> = None;

        let bvh_tris = self.traverse_bvh(ray);

        // self.triangles.iter().for_each(|triangle| {
        bvh_tris.iter().for_each(|triangle| {
        unsafe {
            if let Some(current_intersection) = triangle.read().unwrap_unchecked().intersects_ray(&self.origin, &ray, pixel_pointer) {
                if closest_intersection.is_none() {
                    closest_intersection = Some(current_intersection);
                } else {
                    if current_intersection.distance < closest_intersection.unwrap().distance {
                        closest_intersection = Some(current_intersection);
                    }
                }
            }
                unsafe {
                    (*pixel_pointer).rendered = 0xFFFFFFFFu32;
                }
        }});
        return closest_intersection;
    }

    pub fn ray_aabb(&self, ray: &Ray) -> bool {
        return self.ray_any_aabb(ray, &self.aabb);
    }

    fn ray_any_aabb(&self, ray: &Ray, aabb: &AABB) -> bool {
        // TODO: This should be additionally translated when object could be rotated

        let inv_d = Vec3::new(
            1.0 / ray.direction.x,
            1.0 / ray.direction.y,
            1.0 / ray.direction.z,
        );

        let t0 = (aabb.min - ray.origin + self.origin) * inv_d;
        let t1 = (aabb.max - ray.origin + self.origin) * inv_d;

        let tmin = t0.min(t1);
        let tmax = t0.max(t1);

        if tmin.max_element() <= tmax.min_element() && tmax.min_element() >= 0.0 {
            return true;
        } else {
            return false;
        }
    }

    pub fn traverse_bvh(&self, ray: &Ray) -> Vec<Arc<RwLock<Triangle>>> {
        let mut tris = Vec::new();
        if self.ray_any_aabb(ray, &self.bvh.aabb) {
            self.traverse_bvh_2(&self.bvh, ray, &mut tris);
        }
        return tris;
    }

    pub fn traverse_bvh_2<'a>(&self, bvh: &'a BVH, ray: &Ray, tris: &mut Vec<Arc<RwLock<Triangle>>>) {
        match &bvh.left {
            BVHNode::Leaf(data) => tris.push(data.clone()),
            BVHNode::Node(data) => {
                if self.ray_any_aabb(ray, &data.aabb) {
                    self.traverse_bvh_2(data, ray, tris);
                }
            },
        };
        match &bvh.right {
            BVHNode::Leaf(data) => tris.push(data.clone()),
            BVHNode::Node(data) => {
                if self.ray_any_aabb(ray, &data.aabb) {
                    self.traverse_bvh_2(data, ray, tris);
                }
            },
        };
    }

    pub fn debug_count_repeated_triangles(&self) {
        let d = self.debug_get_bvh_end(self.bvh.clone());
        let mut c = 0;
        for a in &d {
            for b in &d {
                if (a as *const _) == (b as *const _) {
                    c += 1;
                }
            }
        }
        println!("in total {c} same tris (632 for teapot is ideal)");
        println!("total total tris are {:?}", self.debug_get_bvh_end(self.bvh.clone()).len());
        if let BVHNode::Node(left) = &self.bvh.left {
            if let BVHNode::Node(right) = &self.bvh.left {
                println!("left length: {:?}", self.debug_get_bvh_end(*left.clone()).len());
                println!("right length: {:?}", self.debug_get_bvh_end(*right.clone()).len());
                println!("left:\t{:?}\nright:\t{:?}", left.aabb, right.aabb);
            }
        }
    }

    pub fn debug_get_bvh_end(&self, bvh: BVH) -> Vec<Arc<RwLock<Triangle>>> {
        let mut tris = Vec::new();
        match &bvh.left {
            BVHNode::Leaf(data) => tris.push(data.clone()),
            BVHNode::Node(data) => tris.extend(self.debug_get_bvh_end(*data.clone())),
        };
        match &bvh.right {
            BVHNode::Leaf(data) => tris.push(data.clone()),
            BVHNode::Node(data) => tris.extend(self.debug_get_bvh_end(*data.clone())),
        };
        return tris;
    }
}
