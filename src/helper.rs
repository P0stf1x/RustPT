pub mod polygon;
pub mod object;
pub mod ray;
pub mod screen;
pub mod camera;
pub mod scene;
pub mod texture;
pub mod aabb;

#[inline]
pub fn ARGB4_to_ARGBu32(a: u8, r: u8, g: u8, b: u8) -> u32 {
    ((a as u32) << 24) + ((r as u32) << 16) + ((g as u32) << 8) + b as u32
}
