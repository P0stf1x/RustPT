pub mod pixel;

use std::sync::RwLock;

use pixel::Pixel;

pub struct Texture {
    size_x: usize,
    size_y: usize,
    data: Vec<RwLock<Pixel>>,
}

impl Texture {
    pub fn new(x: usize, y: usize) -> Texture {
        let mut empty_vector = Vec::with_capacity(x*y);
        for _ in 0..(x*y) {
            empty_vector.push(RwLock::new(Pixel::new()));
        }
        Texture { size_x: x, size_y: y, data: empty_vector }
    }

    pub fn new_from_file(file_path: String) -> Texture {
        unimplemented!()
    }

    pub fn get_pixel(&self, x: usize, y: usize) -> &RwLock<Pixel> {
        if x >= self.size_x {
            panic!("Required pixel is outside of the image x dimension. {} >= {}", x, self.size_x);
        }
        if y >= self.size_y {
            panic!("Required pixel is outside of the image y dimension. {} >= {}", y, self.size_y);
        }

        let pixel = &self.data[x + y*self.size_x];
        return pixel;
    }

    pub fn get_pixel_relative(&self, x: f32, y: f32) -> &RwLock<Pixel> {
        // TODO: different repeat modes, e.g. repeating texture; min/max; etc. For now it's just repeats
        let pixel_x = (x * (self.size_x as f32)).floor().rem_euclid(self.size_x as f32) as usize;
        let pixel_y = (y * -(self.size_y as f32)).floor().rem_euclid(self.size_y as f32) as usize; // since V is positive upwards and Y is positive downwards we need to multiply by negative
        return self.get_pixel(pixel_x, pixel_y);
    }
}
