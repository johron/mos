use crate::app::MosId;
use crate::panel::panel::Panel;
use crate::workspace::layout::{FloatingPanel, Layout};
use ratatui::Frame;

pub struct Workspace {
    floating_panel: Option<FloatingPanel>,
    layout: Layout,
}

impl Workspace {
    pub fn new() -> Self {
        Workspace {
            layout: Layout::Tabs {
                tabs: Vec::new(),
                active: None,
            },
            floating_panel: None,
        }
    }

    pub fn add_panel(&mut self, panel: Box<dyn Panel>) {
        match &mut self.layout {
            Layout::Tabs { tabs, active } => {
                let panel_id = panel.id();
                tabs.push(panel);
                *active = Some(panel_id); // Set the newly added panel as active
            }
            _ => {
                eprintln!("Currently only Tabs layout is supported for adding panels");
            }
        }
    }

    pub fn set_floating(&mut self, floating_panel: Option<FloatingPanel>) {
        self.floating_panel = floating_panel;
    }

    pub fn get_active_panel(&self) -> Option<&dyn Panel> {
        self.layout.get_active_panel()
    }

    pub fn get_active_panel_mut(&mut self) -> Option<&mut (dyn Panel + 'static)> {
        self.layout.get_active_panel_mut()
    }

    pub fn get_floating(&mut self) -> &mut Option<FloatingPanel> {
        &mut self.floating_panel
    }

    pub fn render(&self, frame: &mut Frame) {
        // chunks?

        let area = frame.size();
        self.layout.render(frame, area);
    }
}