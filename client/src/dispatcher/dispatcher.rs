use super::Event;

pub struct Dispatcher {}

pub trait IDispatcher {
    fn on(&mut self, event: Event);
}

impl Dispatcher {
    pub fn new() -> Self {
        Dispatcher {}
    }
}

impl IDispatcher for Dispatcher {
    fn on(&mut self, event: Event) {}
}
