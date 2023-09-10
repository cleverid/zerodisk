use crate::gpu::{GPUVertex, GetGPUData};
use crate::objects::to_gpu_data;
use crate::primitive::{triangle, Color, Point, Triangle};

pub struct Square {
    position: Point,
    color: Color,
    mesh: Vec<Triangle>,
}

impl Square {
    pub fn new(position: Point, color: Color) -> Self {
        Self {
            position,
            color,
            mesh: vec![
                triangle([[100, 100], [0, 0], [0, 100]]),
                triangle([[100, 100], [100, 0], [0, 0]]),
            ],
        }
    }
}

impl GetGPUData for Square {
    fn get_gpu_data(&self) -> Vec<GPUVertex> {
        to_gpu_data(&self.position, &self.mesh, &self.color)
    }
}
