use crate::gpu::{GPUVertex, GetGPUData};
use crate::objects::to_gpu_data;
use crate::primitive::{Color, Point};

pub struct Square<'a> {
    position: Point,
    color: Color,
    points: &'a [Point],
}

impl Square<'_> {
    pub fn new(position: Point, color: Color) -> Self {
        Self {
            position,
            color,
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
        to_gpu_data(&self.position, self.points, &self.color)
    }
}
