use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use tui_textarea::CursorMove;
use crate::input::handle_non_modifier;
use crate::Mosaic;

pub fn handle_mode(mosaic: &mut Mosaic, key_event: KeyEvent) {
    let text_area = &mut mosaic.editors[mosaic.current_editor].text_area;

    if key_event.modifiers.is_empty() {
        handle_non_modifier(mosaic, key_event);
    } else {
        match key_event {
            KeyEvent { code: KeyCode::Left, modifiers: KeyModifiers::CONTROL, .. } => {
                text_area.move_cursor(CursorMove::WordBack)
            },
            KeyEvent { code: KeyCode::Up, modifiers: KeyModifiers::CONTROL, .. } => {
                text_area.move_cursor(CursorMove::Up)
            },
            KeyEvent { code: KeyCode::Down, modifiers: KeyModifiers::CONTROL, .. } => {
                text_area.move_cursor(CursorMove::Down)
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