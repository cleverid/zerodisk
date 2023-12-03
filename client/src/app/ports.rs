use crate::object::Object;
use crate::scene::Scene;

pub trait Component {
    fn init(&self, scene: &mut Scene);
    fn render(&self) -> Vec<Object>;
}
