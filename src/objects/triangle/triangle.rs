use crate::gpu::{GPUVertex, GetGPUData};
use crate::objects::Object;
use crate::primitive::{triangle, Color, Point};

pub struct Triangle {
    object: Object,
}

impl Triangle {
    pub fn new(position: Point, color: Color) -> Self {
        let mesh = vec![triangle([[50, 0], [0, 100], [100, 100]])];
        let object = Object::new(position, color, mesh);
        Self { object }
    }
}

impl GetGPUData for Triangle {
    fn get_gpu_data(&self) -> Vec<GPUVertex> {
        self.object.get_gpu_data()
    }
}
