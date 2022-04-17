
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
        let scale = 1.0 / samples as f64;

        // gamma correction, gamma = 2.0 -> sqrt
        Color {
            r: ((color_vec.x * scale).sqrt() * 255.0).clamp(0.0, 255.0) as u8,
            g: ((color_vec.y * scale).sqrt() * 255.0).clamp(0.0, 255.0) as u8,
            b: ((color_vec.z * scale).sqrt() * 255.0).clamp(0.0, 255.0) as u8,
        }
    }
}
