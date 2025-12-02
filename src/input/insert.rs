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
                //text_area.move_cursor(CursorMove::Back)
            },
            KeyEvent { code: KeyCode::Up, modifiers: KeyModifiers::CONTROL, .. } => {
                text_area.move_cursor(CursorMove::Up)
            },
            KeyEvent { code: KeyCode::Down, modifiers: KeyModifiers::CONTROL, .. } => {
                text_area.move_cursor(CursorMove::Down)
            },
            KeyEvent { code: KeyCode::Right, modifiers: KeyModifiers::CONTROL, .. } => {
                text_area.move_cursor(CursorMove::WordForward)
                //text_area.move_cursor(CursorMove::Forward)
            },
            _ => {
                // KeyEvent is alphabetic? do here
                handle_non_modifier(mosaic, key_event);
            }
        }
    }
}