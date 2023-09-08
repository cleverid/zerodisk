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
