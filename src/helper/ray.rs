use glam::Vec3;

#[derive(Clone, Copy)]
pub struct IntersectionResult {
    pub distance: f32,
    pub uv: Vec3, // FIXME: this is not uv but well ok I guess...
}

#[derive(Clone, Copy)]
pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
}

impl Ray {
    pub fn new(origin: Vec3, direction: Vec3) -> Ray {
        return Ray {
            origin: origin,
            direction: direction.normalize()
        };
    }
}
