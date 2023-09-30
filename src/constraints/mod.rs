mod direct;

use std::collections::HashMap;

pub use direct::*;
use crate::object::Object;
pub trait Constraint {
    fn process(&self, objects: &mut HashMap<String, Object>);
}
#[derive(Clone, Copy, Debug)]
pub enum Axis {
    X = 1,
    Y = 2,
    Z = 3,
}
