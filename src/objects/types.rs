#[derive(Clone, Copy)]
pub struct Point {
    pub x: u32,
    pub y: u32,
}

impl Point {
    pub fn to_gpu(&self) -> [f32; 2] {
        [self.x as f32, self.y as f32]
    }
}

pub struct Color {
    pub r: u32,
    pub g: u32,
    pub b: u32,
    pub a: u32,
}

impl Color {
    pub fn new(r: u32, g: u32, b: u32, a: u32) -> Color {
        Color { r, g, b, a }
    }
    pub fn to_gpu(&self) -> [f32; 3] {
        [self.r as f32, self.g as f32, self.b as f32]
    }
}
