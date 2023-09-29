use crate::object::Object;
use super::{Constraint, Axis};

#[derive(Clone, Debug)]
pub struct DirectConstraint {
    from_id: String,
    from_axis: Axis,
    target_id: String, 
}

impl DirectConstraint {
    pub fn new(from_id: String, target_id: String, from_axis: Axis) -> Self {
        Self{ 
            from_id: String::from(""),
            from_axis: from_axis,
            target_id: String::from(""),
        }
    }
}

impl Constraint for DirectConstraint {
    fn constraint_get_ids(&self) -> Vec<String> {
        todo!()
    }

    fn constraint_process(&self, objects: Vec<&mut Object>) {
        todo!()
    }
}


