use super::Event;
use std::collections::HashMap;

type Callback = fn(event: Event);
type ItemCallback = HashMap<String, Callback>;
type Register = HashMap<Event, ItemCallback>;

pub struct Dispatcher {
    register: Register,
}

pub trait IDispatcher {
    fn on(&mut self, id: String, event: Event, cb: Callback);
}

impl Dispatcher {
    pub fn new() -> Self {
        Dispatcher {
            register: HashMap::new(),
        }
    }
}

impl IDispatcher for Dispatcher {
    fn on(&mut self, id: String, event: Event, cb: Callback) {}
}
