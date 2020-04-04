use hashbrown::HashMap;

use std::hash::Hash;

pub type EventCallback<'callback, T> = Box<dyn FnMut(&mut T) -> () + 'callback + Send + Sync>;

pub trait EventKey: Hash + PartialEq + Eq + Send + Sync {}
impl<T> EventKey for T where T: Hash + PartialEq + Eq + Send + Sync {}

pub trait Events<'callback, K, T>
where
    K: EventKey,
{
    fn on(&mut self, event_name: K, callback: EventCallback<'callback, T>);
    fn off(&mut self, event_name: K);
    fn emit(&mut self, event_name: K, event_data: &mut T);
}

pub struct Emitter<'callback, K, T: 'callback>
where
    K: EventKey,
{
    events: HashMap<K, Vec<EventCallback<'callback, T>>>,
}

impl<'callback, K, T> Emitter<'callback, K, T>
where
    K: EventKey,
{
    pub fn new() -> Emitter<'callback, K, T> {
        Emitter {
            events: HashMap::new(),
        }
    }
}

impl<'callback, K, T> Events<'callback, K, T> for Emitter<'callback, K, T>
where
    K: EventKey,
{
    fn on(&mut self, event_name: K, callback: EventCallback<'callback, T>) {
        let callbacks = self.events.entry(event_name).or_insert(vec![]);
        callbacks.push(callback);
    }

    fn off(&mut self, event_name: K) {
        self.events.remove(&event_name);
    }

    fn emit(&mut self, event_name: K, event_data: &mut T) {
        if let Some(callbacks) = self.events.get_mut(&event_name) {
            for callback in callbacks.iter_mut() {
                callback(event_data)
            }
        }
    }
}
