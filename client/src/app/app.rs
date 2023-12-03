use super::Component;
use crate::scene::Scene;

pub struct Application {}

impl Application {
    pub fn new() -> Application {
        Application {}
    }

    pub fn init(&self, scene: &mut Scene) {}

    pub fn add_component(&mut self, component: impl Component) {}
}
