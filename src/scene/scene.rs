use crate::{
    gpu::{GPUVertex, GetGPUData},
    object::Object,
    primitive::Point,
};

#[derive(Clone, Debug)]
pub struct Scene {
    objects: Vec<Object>,
    handle_callback: Vec<Box<fn(objects: &Vec<Object>)>>,
}

impl Scene {
    pub fn new() -> Self {
        Scene {
            objects: Vec::new(),
            handle_callback: Vec::new(),
        }
    }

    pub fn add(mut self, object: Object) -> Self {
        self.objects.push(object);
        self
    }

    pub fn trace(&mut self, point: Point) {
        for cb in self.handle_callback.iter() {
            cb(&self.objects)
        }
    }

    pub fn handle_trace(mut self, callback: fn(objects: &Vec<Object>)) -> Self {
        self.handle_callback.push(Box::new(callback));
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
