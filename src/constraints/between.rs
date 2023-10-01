use super::Constraint;
use crate::object::{self, Object};
use crate::primitive::{point, Point};
use std::collections::HashMap;

#[derive(Debug, Copy, Clone)]
pub struct BetweenResult {
    pub position: Point,
    pub angle: f32,
    pub distance: f32,
}
type Callback = fn(&mut Object, BetweenResult);

pub struct BetweenConstraint {
    middle_id: String,
    from_id: String,
    target_id: String,
    callback: Callback,
}

impl BetweenConstraint {
    pub fn new(middle_id: String, from_id: String, target_id: String, callback: Callback) -> Self {
        Self {
            from_id,
            middle_id,
            target_id,
            callback,
        }
    }
}

impl Constraint for BetweenConstraint {
    fn process(&self, objects: &mut HashMap<String, Object>) {
        let mut object = objects.get_mut(&self.middle_id).unwrap();
        (self.callback)(
            object,
            BetweenResult {
                position: point(0, 0),
                angle: 12.0,
                distance: 12.0,
            },
        )
    }
}
