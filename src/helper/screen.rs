use derive_new::new;

use crate::helper::ARGB4_to_ARGBu32;

#[derive(new, Clone, Copy)]
pub struct ScreenBuffersPixel {
    #[new(value = "0")]
    pub rendered: u32,
    #[new(value = "f32::INFINITY")]
    pub alpha: f32,
}

pub struct ScreenBuffers {
    pub pixels: Vec<ScreenBuffersPixel>,
}

impl ScreenBuffers {
    pub fn new(width: usize, height: usize) -> Self {
        let size = width*height;
        Self {
            pixels: vec![ScreenBuffersPixel::new(); size]
        }
    }

    pub fn clear(&mut self) {
        self.fill(ScreenBuffersPixel::new());
    }

    pub fn get_rendered(&self) -> Vec<u32> {
        return self.pixels.iter().map(|pixel| {pixel.rendered}).collect();
    }

    pub fn get_depth(&self) -> Vec<u32> {
        let depth: Vec<u8> = self.pixels.iter().map(|pixel| {((-pixel.alpha)+255.) as u8}).collect();
        let mut result = Vec::<u32>::with_capacity(depth.len());
        depth.iter().for_each(|val| {
            result.push(ARGB4_to_ARGBu32(0xFF, *val, *val, *val));
        });
        return result;
    }

    fn fill(&mut self, value: ScreenBuffersPixel) {
        self.pixels.fill(value);
    }
}
