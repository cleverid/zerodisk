use super::{to_gpu_data, Color, Point};
use crate::gpu::{GPUVertex, GetGPUData};

pub struct Square<'a> {
    pivot: Point,
    color: Color,
    points: &'a [Point],
}

impl Square<'_> {
    pub fn new() -> Self {
        Self {
            pivot: Point { x: 10, y: 10 },
            color: Color::new(1, 0, 0, 1),
            points: &[
                Point { x: 100, y: 100 },
                Point { x: 0, y: 0 },
                Point { x: 0, y: 100 },
                Point { x: 100, y: 100 },
                Point { x: 100, y: 0 },
                Point { x: 0, y: 0 },
            ],
        }
    }
}

impl GetGPUData for Square<'_> {
    fn get_gpu_data(&self) -> Vec<GPUVertex> {
        to_gpu_data(&self.pivot, self.points, &self.color)
    }
}
