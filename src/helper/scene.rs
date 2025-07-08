use crate::texture::Texture;
use crate::screen::ScreenBuffers;
use crate::object::Object;
use crate::camera::Camera;

pub struct Scene {
    pub screen: ScreenBuffers,
    pub objects: Vec<Object>,
    pub camera: Camera,
}

impl Scene {
    pub fn render_to_texture(&mut self, camera: &Camera, texture: &Texture) {
        texture.get_pixel_iterator().enumerate().for_each(|(i, pixel)| {
            let x = i % texture.size_x();
            let y = i / texture.size_x();

            let mut p = pixel.write().unwrap();
            p.r_set(x.min(255) as u8);
            p.g_set(y.min(255) as u8);
            drop(p);
        });
    }
}
