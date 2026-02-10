use std::any::Any;
use std::collections::HashMap;
use crate::panel::panel::PanelId;

pub struct DataStore {
    map: HashMap<PanelId, Box<dyn Any>>,
}

impl DataStore {
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }

    pub fn insert<T: 'static>(&mut self, panel_id: PanelId, data: T) {
        self.map.insert(panel_id, Box::new(data));
    }

    pub fn get<T: 'static>(&self, panel_id: &PanelId) -> Option<&T> {
        self.map.get(panel_id)?.downcast_ref::<T>()
    }

    pub fn get_mut<T: 'static>(&mut self, panel_id: &PanelId) -> Option<&mut T> {
        self.map.get_mut(panel_id)?.downcast_mut::<T>()
    }
}