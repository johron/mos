use crossterm::event::{KeyEvent, MouseEvent};

#[derive(Clone, Debug)]
pub enum InputEvent {
    Key(KeyEvent),
    Mouse(MouseEvent),
}

#[derive(Clone, Debug)]
pub enum Event {
    Input(InputEvent),
    Command(String, Vec<String>),
    Tick,
}

impl Event {
    pub fn from_crossterm_event(event: crossterm::event::Event) -> Option<Self> {
        match event {
            crossterm::event::Event::Key(key_event) => Some(Event::Input(InputEvent::Key(key_event))),
            crossterm::event::Event::Mouse(mouse_event) => Some(Event::Input(InputEvent::Mouse(mouse_event))),
            _ => None,
        }
    }
}