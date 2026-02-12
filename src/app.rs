use ratatui::Frame;
use uuid::Uuid;
use crate::event::event::Event;
use crate::plugin_builtin::text_editor::text_editor::TextEditorPlugin;
use crate::system::panel_registry::PanelRegistry;
use crate::system::plugin_registry::PluginRegistry;
use crate::workspace::workspace::Workspace;

#[derive(Eq, Hash, PartialEq, Copy, Clone, Debug)]
pub struct MosId(Uuid);

impl MosId {
    pub fn new() -> Self {
        MosId(Uuid::new_v4())
    }
}

pub struct Mos {
    pub should_quit: bool,
    pub workspaces: Vec<Workspace>,
    pub panel_registry: PanelRegistry,
    pub plugin_registry: PluginRegistry,
}

impl Mos {
    pub fn new() -> Self {
        let mut plugin_registry = PluginRegistry::new();
        let mut panel_registry = PanelRegistry::new();

        plugin_registry.register_plugin(Box::new(TextEditorPlugin::new()));
        plugin_registry.enable_plugins(&mut panel_registry);

        Mos {
            should_quit: false,
            workspaces: Vec::new(),
            panel_registry,
            plugin_registry,
        }
    }

    pub fn update(&mut self) {

    }

    pub fn handle_terminal_event(&mut self, event: crossterm::event::Event) {
        // Only handle key events for global and the current active panel.

        let mos_event = Event::from_crossterm_event(event);
        if let Some(ev) = mos_event {
            self.plugin_registry.handle_plugins_events(ev);
        }
    }

    pub fn render(&mut self, _frame: &mut Frame) {
        // Render UI here
    }
}