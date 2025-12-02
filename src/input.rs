use crate::{Mode, Mosaic};
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyModifiers};
use std::io::Error;
use tui_textarea::{CursorMove, Input, Key};

mod normal;
mod insert;
mod command;

use std::sync::{Mutex, OnceLock};
use std::time::{Instant, Duration};

static MOS_PREFIX: OnceLock<Mutex<Option<Instant>>> = OnceLock::new();

pub fn handle(mosaic: &mut Mosaic) -> Result<(), Error> {
    if event::poll(std::time::Duration::from_millis(10))? {
        if let Event::Key(key_event) = event::read()? {
            process_key(mosaic, key_event);
        }
    }

    Ok(())
}

fn process_key(mosaic: &mut Mosaic, key: KeyEvent) {
    const PREFIX_TIMEOUT: Duration = Duration::from_millis(500);

    let prefix_lock = MOS_PREFIX.get_or_init(|| Mutex::new(None));
    let mut guard = prefix_lock.lock().unwrap();

    // If Tab pressed, set prefix timestamp and wait for next key
    if key.code == KeyCode::F(12) {
        // *guard = Some(Instant::now()); disable prefix for now
        return;
    }

    // If prefix active and next key within timeout, handle combos
    if let Some(ts) = *guard {
        if ts.elapsed() <= PREFIX_TIMEOUT {
            *guard = None; // consume prefix

            match key.code {
                KeyCode::Right => {
                    let len = mosaic.editors.len();
                    if len > 0 {
                        mosaic.current_editor = (mosaic.current_editor + 1) % len;
                    }
                    return;
                }
                KeyCode::Left => {
                    let len = mosaic.editors.len();
                    if len > 0 {
                        mosaic.current_editor = (mosaic.current_editor + len - 1) % len;
                    }
                    return;
                }
                _ => {
                    *guard = None; // unrecognized combo, reset prefix
                }
            }
        } else {
            // prefix timed out
            *guard = None;
        }
    }

    // Fallback to normal mode-specific handling
    match mosaic.mode {
        Mode::Normal => normal::handle_mode(mosaic, key),
        Mode::Insert => insert::handle_mode(mosaic, key),
        Mode::Command => command::handle_mode(mosaic, key),
    }
}

fn handle_non_modifier(mosaic: &mut Mosaic, key_event: KeyEvent) {
    let text_area = &mut mosaic.editors[mosaic.current_editor].text_area;
    match key_event.code {
        KeyCode::Esc => mosaic.set_mode(Mode::Normal),
        KeyCode::Tab => {
            text_area.input(Input {
                key: Key::Tab,
                ctrl: false,
                alt: false,
                shift: false,
            });
        },
        KeyCode::BackTab => {
            let row = text_area.cursor().0;
            let current_line = text_area.lines()[row].as_str();
            let leading_spaces = current_line.chars().take_while(|c| *c == ' ').count();
            let to_remove = std::cmp::min(4, leading_spaces);

            for _ in 0..to_remove {
                let (r, col) = text_area.cursor();
                if col == 0 { break; }
                let prev_char = text_area.lines()[r].chars().nth(col.saturating_sub(1)).unwrap_or('\0');
                if prev_char == ' ' {
                    text_area.delete_char();
                } else {
                    break;
                }
            }
        },

        KeyCode::Char(c) => {
            text_area.input(Input {
                key: Key::Char(c),
                ctrl: false,
                alt: false,
                shift: false,
            });
        },
        KeyCode::Left => text_area.move_cursor(CursorMove::Back),
        KeyCode::Up => text_area.move_cursor(CursorMove::Up),
        KeyCode::Down => text_area.move_cursor(CursorMove::Down),
        KeyCode::Right => text_area.move_cursor(CursorMove::Forward),

        KeyCode::Enter => {
            text_area.insert_newline();

            let row = text_area.cursor().0.saturating_sub(1);
            let current_line = text_area.lines()[row].as_str();

            let indent: String = current_line.chars().take_while(|c| c.is_whitespace()).collect();

            for _ in 0..indent.len() {
                text_area.input(Input {
                    key: Key::Char(' '),
                    ctrl: false,
                    alt: false,
                    shift: false,
                });
            }
        },

        KeyCode::Backspace => {
            text_area.delete_char();
        },
        _ => {}
    }
}