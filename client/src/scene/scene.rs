use std::collections::{HashMap, HashSet};

use crate::{
    constraints::Constraint,
    gpu::{GPUVertex, GetGPUData},
    object::Object,
    primitive::{point, Point},
};

use super::tracer::Tracer;

pub struct Scene {
    pub tracer: Tracer,
    objects: HashMap<String, Object>,
    constraints: Vec<Box<dyn Constraint>>,
    traced: HashSet<String>,
    dragged: HashSet<String>,
    mouse_cursor: Point,
}

impl Scene {
    pub fn new() -> Self {
        Scene {
            objects: HashMap::new(),
            tracer: Tracer::new(),
            traced: HashSet::new(),
            dragged: HashSet::new(),
            mouse_cursor: point(0.0, 0.0),
            constraints: Vec::new(),
        }
    }

    pub fn add_objects(&mut self, objects: Vec<Object>) {
        for object in objects.iter() {
            self.objects.insert(object.id.clone(), object.clone());
            self.tracer.index(object.id.clone(), object.get_mesh());
        }
    }

    pub fn add_constraint(&mut self, constraint: impl Constraint + 'static) {
        self.constraints.push(Box::new(constraint));
    }

    pub fn add_constraints(&mut self, constraints: Vec<Box<dyn Constraint + 'static>>) {
        for con in constraints {
            self.constraints.push(con);
        }
    }

    pub fn set_mouse_click_left(&mut self, clicked: bool) {
        if clicked {
            self.dragged = self.traced.clone();
        } else {
            self.dragged = HashSet::new();
        }
    }

    pub fn set_mouse_position(&mut self, point: Point) -> bool {
        let change_trace = self.trace(point.clone());
        let change_drag = self.drag(point.clone());
        self.mouse_cursor = point;
        change_trace || change_drag
    }

    pub fn process(&mut self) {
        for con in self.constraints.iter_mut() {
            con.process(&mut self.objects);
        }
    }

    fn trace(&mut self, trace_point: Point) -> bool {
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

    fn drag(&mut self, trace_point: Point) -> bool {
        let drag = trace_point - self.mouse_cursor;
        let mut changed = false;
        if !drag.is_zero() && self.dragged.len() > 0 {
            changed = true;
            for id in self.dragged.clone().iter() {
                self.get_object_mut(id).unwrap().move_object(drag.clone());
                let object = self.get_object(id).unwrap();
                self.tracer.index(id.clone(), object.get_mesh());
            }
        }
        changed
    }

    fn get_object(&self, object_id: &String) -> Option<&Object> {
        self.objects.get(object_id)
    }

    fn get_object_mut(&mut self, object_id: &String) -> Option<&mut Object> {
        self.objects.get_mut(object_id)
    }

    fn mark_traced(&mut self, traced_ids: &HashSet<String>, status: bool) {
        for id in traced_ids.iter() {
            self.get_object_mut(id).unwrap().set_highlighted(status);
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
