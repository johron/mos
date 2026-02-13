use crate::event::event::Event;
use crate::plugin::plugin::Plugin;
use crate::system::panel_registry::PanelRegistry;

pub struct PluginRegistry {
    plugins: Vec<Box<dyn Plugin>>,
}

impl PluginRegistry {
    pub fn new() -> Self {
        Self {
            plugins: Vec::new(),
        }
    }

    pub fn register_plugin(&mut self, plugin: Box<dyn Plugin>) {
        self.plugins.push(plugin);
    }

    pub fn get_plugins(&self) -> &Vec<Box<dyn Plugin>> {
        &self.plugins
    }
    
    pub fn enable_plugins(&mut self, panel_registry: &mut PanelRegistry) {
        for plugin in self.plugins.iter_mut() {
            if let Err(e) = plugin.enable(panel_registry) {
                eprintln!("Failed to enable plugin {}: {}", plugin.name(), e);
            }
        }
    }
    
    pub fn handle_plugins_events(&mut self, event: Event) {
        for plugin in self.plugins.iter_mut() {
            plugin.handle_event(event.clone()).unwrap_or_else(|e| {
                eprintln!("Error handling event in plugin {}: {}", plugin.name(), e);
            });
        }
    }
}