use super::Constraint;
use crate::helpers::{angle_direct, distance, middle};
use crate::object::{self, Object};
use crate::primitive::{point, Point};
use std::collections::HashMap;

#[derive(Debug, Copy, Clone)]
pub struct BetweenResult {
    pub middle: Point,
    pub angle: f32,
    pub distance: u32,
}
type Callback = fn(&mut Object, BetweenResult);

pub struct BetweenConstraint {
    constraint_id: String,
    from_id: String,
    target_id: String,
    callback: Callback,
}

impl BetweenConstraint {
    pub fn new(
        constraint_id: String,
        from_id: String,
        target_id: String,
        callback: Callback,
    ) -> Self {
        Self {
            constraint_id,
            from_id,
            target_id,
            callback,
        }
    }
}

impl Constraint for BetweenConstraint {
    fn process(&self, objects: &mut HashMap<String, Object>) {
        let from = objects.get(&self.from_id).unwrap().position;
        let target = objects.get(&self.target_id).unwrap().position;
        let mut constraint = objects.get_mut(&self.constraint_id).unwrap();
        (self.callback)(
            constraint,
            BetweenResult {
                middle: middle(from, target),
                angle: angle_direct(from, target),
                distance: distance(from, target),
            },
        )
    }
}
