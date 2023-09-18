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
        self.objects.push(object.clone());
        self.tracer.index(object.id.clone(), object.get_mesh());
        self
    }

    pub fn mark_traced(&mut self, traced_ids: Vec<String>) -> bool {
        let mut changed = false;
        for object in self.objects.iter_mut() {
            let mut off = false;
            let mut on = false;
            if traced_ids.contains(&object.id) {
                on = object.set_highlighted(true);
            } else {
                off = object.set_highlighted(false);
            }
            if off || on {
                changed = true;
            }
        }
        changed
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
