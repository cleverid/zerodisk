use super::Component;
use crate::scene::Scene;

pub struct Application {
    components: Vec<Box<dyn Component>>,
}

impl Application {
    pub fn new() -> Application {
        Application { components: vec![] }
    }

    pub fn init(&self, scene: &mut Scene) {
        for component in &self.components {
            component.init(scene);
            scene.add_objects(component.render());
        }
        scene.process();
    }

    pub fn add_component(&mut self, component: impl Component + 'static) {
        self.components.push(Box::new(component));
    }
}
