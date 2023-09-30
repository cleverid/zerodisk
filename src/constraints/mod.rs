mod direct;

use std::collections::HashMap;

use crate::object::Object;
pub use direct::*;
pub trait Constraint {
    fn process(&self, objects: &mut HashMap<String, Object>);
}
#[derive(Clone, Copy, Debug)]
pub enum Axis {
    X = 1,
    Y = 2,
    Z = 3,
}
