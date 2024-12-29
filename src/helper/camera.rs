use glam::{ Vec2, Vec3, Quat };
use derive_new::new;
use rayon::prelude::*;

use crate::ray::Ray;
use crate::object::Object;
use crate::screen::{ ScreenBuffers, ScreenBuffersPixel };

#[derive(new)]
pub struct Camera {
    origin: Vec3,
    rotation: Vec3, // Yaw, Pitch, Roll // from -Z
    #[new(value = "90.")]
    #[allow(dead_code)] // will be used in future to calculate fov
    pub fov: f64,
    #[new(value = "false")]
    pub orthographic: bool,
}

impl Camera {
    pub fn render(&self, objects: &Vec<Object>, screen: &mut ScreenBuffers) {
        use crate::{ WIDTH, HEIGHT };
        screen.pixels.par_iter_mut().enumerate().for_each(|(i, pixel)| {
            let x = i % WIDTH;
            let y = i / WIDTH;
            
            let rel_x = x as f32 / (WIDTH as f32/2.0) - 1.0;
            let rel_y = -(y as f32 / (HEIGHT as f32/2.0) - 1.0);
            // Thought: is it neseccary to store relative x and then multiply it by aspect ratio, or is better to right away calculate it with aspect ratio?
            
            let pixel_pointer = pixel as *mut ScreenBuffersPixel;
            let ray = self.generate_ray(Vec2::new(rel_x, rel_y));

            objects.iter().for_each(|object| {
                object.calculate_intersection(&ray, pixel_pointer);
            });
        });
    }

    fn generate_ray(&self, offset: Vec2) -> Ray {
        if !self.orthographic {
            self.generate_ray_perspective(offset)
        } else {
            self.generate_ray_orthographic(offset)
        }
    }

    fn generate_ray_perspective(&self, offset: Vec2) -> Ray {
        use crate::ASPECT_RATIO;
        let (relative_x, relative_y) = (offset.x, offset.y);
        let ray_direction = {
            //          TODO: This is hardcoded and incorrect (at least for fov >= 180deg)
            self.front() + // FOV 90
            self.up()*relative_y +
            self.right()*relative_x*ASPECT_RATIO
        };
        return Ray::new(self.origin, ray_direction);
    }

    fn generate_ray_orthographic(&self, offset: Vec2) -> Ray {
        use crate::ASPECT_RATIO;
        let (relative_x, relative_y) = (offset.x, offset.y);
        // TODO: Orthographic camera scale instead of const
        const CAMERA_SCALE: f32 = 2.56;
        let ray_relative_position = self.right()*CAMERA_SCALE*relative_x*ASPECT_RATIO as f32 + self.up()*CAMERA_SCALE*relative_y as f32;
        let ray_origin = self.origin + ray_relative_position;
        return Ray::new(ray_origin, self.front());
    }

    pub fn translate(&mut self, offset: Vec3) {
        self.origin += offset;
    }
    pub fn rotate(&mut self, angle: Vec3) {
        self.rotation += angle;
    }

    fn calculate_rotation(&self, dir: Vec3) -> Vec3 {
        let deg_to_rad = 57.2957795131;
        let yaw = Quat::from_axis_angle(Vec3::Y, self.yaw()/deg_to_rad);
        let pitch = Quat::from_axis_angle(yaw*Vec3::X, self.pitch()/deg_to_rad);
        let roll = Quat::from_axis_angle(pitch*(yaw*Vec3::Z), self.roll()/deg_to_rad);
        return roll*(pitch*(yaw*dir));
    }
    pub fn front(&self) -> Vec3 {self.calculate_rotation(Vec3::NEG_Z)}
    pub fn back(&self) -> Vec3 {self.calculate_rotation(Vec3::Z)}
    pub fn up(&self) -> Vec3 {self.calculate_rotation(Vec3::Y)}
    #[allow(dead_code)] // I guess it might be used sometime ¯\_(ツ)_/¯
    pub fn down(&self) -> Vec3 {self.calculate_rotation(Vec3::NEG_Y)}
    pub fn right(&self) -> Vec3 {self.calculate_rotation(Vec3::X)}
    pub fn left(&self) -> Vec3 {self.calculate_rotation(Vec3::NEG_X)}

    pub fn yaw(&self) -> f32 {return self.rotation.x;}
    pub fn pitch(&self) -> f32 {return self.rotation.y;}
    pub fn roll(&self) -> f32 {return self.rotation.z;}
}
