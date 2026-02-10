use crossterm::event::Event;
use ratatui::Frame;
use crate::workspace::workspace::Workspace;

pub struct Mos {
    pub should_quit: bool,
    pub workspaces: Vec<Workspace>,
}

impl Mos {
    pub fn new() -> Self {
        Mos {
            should_quit: false,
            workspaces: Vec::new(),
        }
    }

    pub fn update(&mut self) {
        // Update app state here
    }

    pub fn handle_terminal_event(&mut self, event: Event) {
        // Handle terminal events here
    }

    pub fn render(&mut self, _frame: &mut Frame) {
        // Render UI here
    }
}