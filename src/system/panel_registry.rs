use std::collections::HashMap;
use crate::app::MosId;

pub struct PanelRegistry {
    panels: HashMap<MosId, MosId>, // (plugin_id, panel_id)
}

impl PanelRegistry {
    pub fn new() -> Self {
        Self {
            panels: HashMap::new(),
        }
    }

    pub fn register_panel(&mut self, plugin_id: MosId, panel_id: MosId) {
        self.panels.insert(panel_id, plugin_id);
    }
    
    pub fn unregister_panels_by_plugin(&mut self, plugin_id: &MosId) {
        self.panels.retain(|_, p_id| p_id != plugin_id);
    }
    
    pub fn get_panels(&self) -> &HashMap<MosId, MosId> {
        &self.panels
    }
    
    pub fn get_panels_by_plugin(&self, plugin_id: &MosId) -> Vec<MosId> {
        self.panels.iter()
            .filter_map(|(panel_id, p_id)| if p_id == plugin_id { Some(panel_id.clone()) } else { None })
            .collect()
    } // need some way of instantiating the panel from id, maybe in plugin registry
}