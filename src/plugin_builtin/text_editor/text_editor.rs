use crate::app::MosId;
use crate::panel::panel::PanelCtor;
use crate::plugin::plugin::{Plugin, PluginRegistration};
use crate::plugin_builtin::text_editor::editor_panel::EditorPanel;
use crate::system::panel_registry::PanelRegistry;

pub struct TextEditorPlugin {
    pub id: MosId,
}

impl TextEditorPlugin {
    pub fn new() -> Self {
        Self {
            id: MosId::new(),
        }
    }
}

impl Plugin for TextEditorPlugin {
    fn id(&self) -> MosId {
        self.id
    }
    
    fn name(&self) -> &str {
        "Text Editor"
    }

    fn version(&self) -> &str {
        "0.1.0"
    }

    fn description(&self) -> &str {
        "The built-in text editor plugin for Mos"
    }

    fn enable(&mut self, panel_registry: &mut PanelRegistry) -> Result<(), String> {
        let panel_id = MosId::new();
        panel_registry.register_panel(self.id(), panel_id, || Box::new(EditorPanel::new()));
        
        println!("Enabled Text Editor Plugin with panel id: {:?}", panel_id);
        
        Ok(())
    }

    fn disable(&mut self) -> PluginRegistration {
        todo!()
    }

    fn handle_event(&mut self, _event: crate::event::event::Event) -> Result<(), String> {
        println!("Text Editor Plugin received event: {:?}", _event);
        Ok(())
    }

}