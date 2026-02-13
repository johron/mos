use crate::app::MosId;
use crate::panel::panel::Panel;
use crate::workspace::layout::{FloatingPanel, Layout};
use ratatui::Frame;

pub struct Workspace {
    layout: Layout,
    floating_panels: Vec<FloatingPanel>,
}

impl Workspace {
    pub fn new() -> Self {
        Workspace {
            layout: Layout::Tabs {
                tabs: Vec::new(),
                active: None,
            },
            floating_panels: Vec::new(),
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

    pub fn get_active_panel(&self) -> Option<&dyn Panel> {
        self.layout.get_active_panel()
    }

    pub fn get_active_panel_mut(&mut self) -> Option<&mut (dyn Panel + 'static)> {
        self.layout.get_active_panel_mut()
    }

    pub fn render(&self, frame: &mut Frame) {
        // chunks?

        let area = frame.size();
        self.layout.render(frame, area);
    }
}