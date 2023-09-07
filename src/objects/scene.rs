use crate::gpu::{GetGPUData, GPUVertex};

pub struct Scene<'a> {
    objects: Vec<&'a dyn GetGPUData>,
}

impl Scene<'_> {
    pub fn new() -> Self {
        Scene { objects: Vec::new() }
    }
    pub fn add(&mut self, object: &dyn GetGPUData) -> &Self {
        self.objects.push(object);
        self
    }
}

impl GetGPUData for Scene<'_> {
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
