mod between;
mod direct;

pub use between::*;
pub use direct::*;

use crate::object::Object;
use std::collections::HashMap;
pub trait Constraint {
    fn process(&self, objects: &mut HashMap<String, Object>);
}

#[derive(Clone, Copy, Debug)]
pub enum Axis {
    X = 1,
    Y = 2,
}
