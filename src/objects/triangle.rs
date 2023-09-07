use super::{Color, Point, to_gpu_data};
use crate::gpu::{GetGPUData, GPUVertex};

pub struct Triangle<'a> {
    pivot: Point,
    color: Color,
    points: &'a [Point],
}

impl Triangle<'_> {
    pub fn new() -> Self {
        Self {
            pivot: Point { x: 200, y: 200 },
            color: Color::new(1, 1, 0, 0),
            points: &[
                Point { x: 50, y: 0 },
                Point { x: 0, y: 100 },
                Point { x: 100, y: 100 },
            ],
        }
    }
}

impl GetGPUData for Triangle<'_> {
    fn get_gpu_data(&self) -> Vec<GPUVertex> {
        to_gpu_data(&self.pivot, self.points, &self.color)
    }
}
