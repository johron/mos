use crossterm::event::Event;
use ratatui::Frame;

pub struct Mos {
    
}

impl Mos {
    pub fn new() -> Self {
        Mos {}
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