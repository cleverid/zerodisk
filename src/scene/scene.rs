use crate::{
    gpu::{GPUVertex, GetGPUData},
    object::Object,
};

use super::tracer::Tracer;

#[derive(Clone, Debug)]
pub struct Scene {
    pub tracer: Tracer,
    objects: Vec<Object>,
}

impl Scene {
    pub fn new() -> Self {
        Scene {
            objects: Vec::new(),
            tracer: Tracer::new(),
        }
    }

    pub fn add(mut self, object: Object) -> Self {
        self.objects.push(object);
        self
    }

    pub fn mark_traced(&mut self, _traced_ids: Vec<String>) -> bool {
        false
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
