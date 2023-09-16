#[derive(Clone, Copy, Debug)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    pub fn new(r: u8, g: u8, b: u8, a: u8) -> Color {
        Color { r, g, b, a }
    }
    pub fn to_gpu(&self) -> [f32; 3] {
        [gpu_float(self.r), gpu_float(self.g), gpu_float(self.b)]
    }
}

fn gpu_float(value: u8) -> f32 {
    value as f32 / 256.0
}
