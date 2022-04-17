
use crate::Vec3;


#[derive(Debug, Clone, Copy)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {

    pub fn new() -> Color { 
        Color { r: 0, g: 0, b: 0 }
    }

    pub fn normalize(color_vec: Vec3, samples: usize) -> Color {
        Color {
            r: ((color_vec.x / samples as f64) * 255.0) as u8,
            g: ((color_vec.y / samples as f64) * 255.0) as u8,
            b: ((color_vec.z / samples as f64) * 255.0) as u8,
        }
    }
}
