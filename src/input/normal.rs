use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use tui_textarea::CursorMove;
use crate::{Mode, Mosaic};

pub fn handle_mode(mosaic: &mut Mosaic, key_event: KeyEvent) {
    let text_area = &mut mosaic.editors[mosaic.current_editor].text_area;

    if key_event.modifiers.is_empty() {
        match key_event.code {
            KeyCode::Esc => {
                mosaic.command.result = None;
            },
            KeyCode::Char('i') => mosaic.set_mode(Mode::Insert),
            KeyCode::Char('q') => {
                mosaic.command.result = None;
                mosaic.set_mode(Mode::Command)
            },

            KeyCode::Char('j') | KeyCode::Left => text_area.move_cursor(CursorMove::Back),
            KeyCode::Char('k') | KeyCode::Up => text_area.move_cursor(CursorMove::Up),
            KeyCode::Char('l') | KeyCode::Down => text_area.move_cursor(CursorMove::Down),
            KeyCode::Char('ø') | KeyCode::Right => text_area.move_cursor(CursorMove::Forward),

            _ => {}
        }
    } else {
        match key_event {
            KeyEvent { code: KeyCode::Char('j') | KeyCode::Left, modifiers: KeyModifiers::CONTROL, .. } => {
                text_area.move_cursor(CursorMove::WordBack)
            },
            KeyEvent { code: KeyCode::Char('k') | KeyCode::Up, modifiers: KeyModifiers::CONTROL, .. } => {
                text_area.move_cursor(CursorMove::Up)
            },
            KeyEvent { code: KeyCode::Char('l') | KeyCode::Down, modifiers: KeyModifiers::CONTROL, .. } => {
                text_area.move_cursor(CursorMove::Down)
            },
            KeyEvent { code: KeyCode::Char('ø') | KeyCode::Right, modifiers: KeyModifiers::CONTROL, .. } => {
                text_area.move_cursor(CursorMove::WordForward)
            },
            _ => {

            }
        }
    }
}