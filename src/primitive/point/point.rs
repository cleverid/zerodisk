#[derive(Clone, Copy, Debug)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub fn to_gpu(&self) -> [f32; 2] {
        [self.x as f32, self.y as f32]
    }

    pub fn to_absolute(&self, pivot: Point) -> Point {
        let x = pivot.x + self.x;
        let y = pivot.y + self.y;
        return Point { x, y };
    }
}

impl Default for Point {
    fn default() -> Self {
        Point { x: 0, y: 0 }
    }
}
