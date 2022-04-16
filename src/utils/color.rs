
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

}
