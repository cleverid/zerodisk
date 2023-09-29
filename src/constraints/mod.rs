mod direct;

pub use direct::*;
use crate::object::Object;
pub trait Constraint {
    fn constraint_get_ids(&self) -> Vec<String>;
    fn constraint_process(&self, objects: Vec<&mut Object>);
}
#[derive(Clone, Copy, Debug)]
pub enum Axis {
    X = 1,
    Y = 2,
    Z = 3,
}
