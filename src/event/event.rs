use crossterm::event::{KeyCode, KeyEvent, KeyModifiers, MouseEvent};

#[derive(Clone, Debug)]
pub enum InputEvent {
    Keyboard(Vec<String>), // convert to Vec<String>
    Mouse(MouseEvent),
}

#[derive(Clone, Debug)]
pub enum Event {
    Input(InputEvent),
    Command(String, Vec<String>),
    Tick,
}

impl Event {
    pub fn keyboard_input_event_from_crossterm_key(event: KeyEvent) -> Vec<String> {
        fn modifier_name(modifier: KeyModifiers) -> String {
            let dbg = format!("{:?}", modifier);

            dbg.trim_start_matches("KeyModifiers(")
                .trim_end_matches(')')
                .to_lowercase()
        }

        let mut keyboard_vec: Vec<String> = Vec::new();

        // Iterate all modifier flags
        let all_modifiers = [
            KeyModifiers::SHIFT,
            KeyModifiers::CONTROL,
            KeyModifiers::ALT,
            KeyModifiers::SUPER,
            KeyModifiers::HYPER,
            KeyModifiers::META,
        ];

        for modifier in all_modifiers {
            if event.modifiers.contains(modifier) {
                keyboard_vec.push(modifier_name(modifier));
            }
        }

        // Key code → string programmatically
        let key_str = match event.code {
            KeyCode::BackTab => String::from("tab"),
            KeyCode::F(n) => format!("f{}", n),
            KeyCode::Char(c) => c.to_string().to_lowercase(),
            other => format!("{:?}", other).to_lowercase(),
        };

        keyboard_vec.push(key_str);

        keyboard_vec
    }

    pub fn from_crossterm_event(event: crossterm::event::Event) -> Option<Self> {
        match event {
            crossterm::event::Event::Key(key_event) => {
                let keyboard_vec = Event::keyboard_input_event_from_crossterm_key(key_event);

                Some(Event::Input(InputEvent::Keyboard(keyboard_vec)))
            },
            crossterm::event::Event::Mouse(mouse_event) => Some(Event::Input(InputEvent::Mouse(mouse_event))),
            _ => None,
        }
    }
}