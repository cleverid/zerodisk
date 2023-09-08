use crate::gpu::{GPUVertex, GetGPUData};

pub struct Scene {
    objects: Vec<Box<dyn GetGPUData>>,
}

impl Scene {
    pub fn new() -> Self {
        Scene {
            objects: Vec::new(),
        }
    }
    pub fn add(&mut self, object: impl GetGPUData + 'static) -> &mut Self {
        self.objects.push(Box::new(object));
        self
    }
}

impl GetGPUData for Scene {
    fn get_gpu_data(&self) -> Vec<GPUVertex> {
        let mut result: Vec<GPUVertex> = Vec::new();
        for obj in self.objects.iter() {
            let data = obj.get_gpu_data();
            for item in data {
                result.push(item)
            }
        }
        result
    }
}
