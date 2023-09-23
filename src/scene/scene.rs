use std::collections::{HashMap, HashSet};

use crate::{
    gpu::{GPUVertex, GetGPUData},
    object::Object,
    primitive::Point,
};

use super::tracer::Tracer;

#[derive(Clone, Debug)]
pub struct Scene {
    pub tracer: Tracer,
    objects: HashMap<String, Object>,
    traced: HashSet<String>,
}

impl Scene {
    pub fn new() -> Self {
        Scene {
            objects: HashMap::new(),
            tracer: Tracer::new(),
            traced: HashSet::new(),
        }
    }

    pub fn add(mut self, object: Object) -> Self {
        self.objects.insert(object.id.clone(), object.clone());
        self.tracer.index(object.id.clone(), object.get_mesh());
        self
    }

    pub fn trace(&mut self, trace_point: Point) -> bool {
        let traced = HashSet::from_iter(self.tracer.trace(trace_point));
        let changed = traced != self.traced;
        if changed {
            let traced_off = self.traced.difference(&traced).map(|i| i.clone()).collect();
            let traced_on = traced.difference(&self.traced).map(|i| i.clone()).collect();
            self.mark_traced(&traced_off, false);
            self.mark_traced(&traced_on, true);
            self.traced = traced;
        }
        changed
    }

    fn get_object_by_id(&mut self, object_id: &String) -> Option<&mut Object> {
        self.objects.get_mut(object_id)
    }

    fn mark_traced(&mut self, traced_ids: &HashSet<String>, status: bool) {
        for id in traced_ids.iter() {
            self.get_object_by_id(id).unwrap().set_highlighted(status);
        }
    }
}

impl GetGPUData for Scene {
    fn get_gpu_data(&self) -> Vec<GPUVertex> {
        let mut result: Vec<GPUVertex> = Vec::new();
        for (_, obj) in self.objects.iter() {
            let data = obj.get_gpu_data();
            for item in data {
                result.push(item)
            }
        }
        result
    }
}
