use super::{Axis, Constraint};
use crate::object::Object;
use std::collections::HashMap;
use std::f32::consts::PI;

#[derive(Clone, Debug)]
pub struct DirectConstraint {
    from_id: String,
    from_axis: Axis,
    target_id: String,
}

impl DirectConstraint {
    pub fn new(from_id: String, target_id: String, from_axis: Axis) -> Self {
        Self {
            from_id,
            from_axis,
            target_id,
        }
    }
}

impl Constraint for DirectConstraint {
    fn process(&self, objects: &mut HashMap<String, Object>) {
        let from = objects.get(&self.from_id).unwrap().position;
        let target = objects.get(&self.target_id).unwrap().position;
        let diff = target - from;
        let angle = (diff.y as f32).atan2(diff.x as f32) + PI / 2.0;
        let mut from_object = objects.get_mut(&self.from_id).unwrap();
        from_object.rotate(angle)
    }
}
