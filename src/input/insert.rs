use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use crate::editor::CursorMove;
use crate::input::handle_non_modifier;
use crate::Mosaic;

pub fn handle_mode(mosaic: &mut Mosaic, key_event: KeyEvent) {
    let text_area = &mut mosaic.editors[mosaic.current_editor];

    if key_event.modifiers.is_empty() {
        handle_non_modifier(mosaic, key_event);
    } else {
        match key_event {
            KeyEvent { code: KeyCode::Left, modifiers: KeyModifiers::CONTROL, .. } => {
                text_area.move_cursor(CursorMove::WordBack)
            },
            KeyEvent { code: KeyCode::Up, modifiers: KeyModifiers::CONTROL, .. } => {
                text_area.scroll_up();
            },
            KeyEvent { code: KeyCode::Down, modifiers: KeyModifiers::CONTROL, .. } => {
                text_area.scroll_down();
            },
            KeyEvent { code: KeyCode::Right, modifiers: KeyModifiers::CONTROL, .. } => {
                text_area.move_cursor(CursorMove::WordForward)
            },
            _ => {
                // KeyEvent is alphabetic? do here
                handle_non_modifier(mosaic, key_event);
            }
        }
    }
}