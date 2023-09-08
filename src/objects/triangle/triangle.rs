use crate::gpu::{GPUVertex, GetGPUData};
use crate::objects::{to_gpu_data, Color, Point};

pub struct Triangle<'a> {
    pivot: Point,
    color: Color,
    points: &'a [Point],
}

impl Triangle<'_> {
    pub fn new(pivot: Point, color: Color) -> Self {
        Self {
            pivot,
            color,
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
