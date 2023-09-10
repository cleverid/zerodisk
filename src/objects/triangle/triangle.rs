use crate::gpu::{GPUVertex, GetGPUData};
use crate::objects::to_gpu_data;
use crate::primitive::{triangle, Color, Point, Triangle as TriangleMesh};

pub struct Triangle {
    position: Point,
    color: Color,
    mesh: Vec<TriangleMesh>,
}

impl Triangle {
    pub fn new(position: Point, color: Color) -> Self {
        Self {
            position,
            color,
            mesh: vec![triangle([[50, 0], [0, 100], [100, 100]])],
        }
    }
}

impl GetGPUData for Triangle {
    fn get_gpu_data(&self) -> Vec<GPUVertex> {
        to_gpu_data(&self.position, &self.mesh, &self.color)
    }
}
