use crate::gpu::{GPUVertex, GetGPUData};
use crate::objects::Object;
use crate::primitive::{triangle, Color, Point};

pub struct Square {
    object: Object,
}

impl Square {
    pub fn new(position: Point, color: Color) -> Self {
        let mesh = vec![
            triangle([[100, 100], [0, 0], [0, 100]]),
            triangle([[100, 100], [100, 0], [0, 0]]),
        ];
        let object = Object::new(position, color, mesh);
        Self { object }
    }
}

impl GetGPUData for Square {
    fn get_gpu_data(&self) -> Vec<GPUVertex> {
        self.object.get_gpu_data()
    }
}
