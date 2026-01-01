use crate::{Mode, Mosaic};
use crossterm::event::{self, Event, KeyCode, KeyEvent};
use std::io::Error;

mod normal;
mod insert;
mod command;

use crate::editor::CursorMove;
use std::sync::{Mutex, OnceLock};
use std::time::{Duration, Instant};

static MOS_PREFIX: OnceLock<Mutex<Option<Instant>>> = OnceLock::new();

pub fn handle(mosaic: &mut Mosaic) -> Result<(), Error> {
    if event::poll(Duration::from_millis(10))? {
        if let Event::Key(key_event) = event::read()? {
            mosaic.toast = None;

            process_key(mosaic, key_event).expect("TODO: panic message");
        }
        //if let Event::Mouse(mouse_event) = event::read()? {
        //    // process mouse event
        //}
    }

    Ok(())
}

fn process_key(mosaic: &mut Mosaic, key: KeyEvent) -> Result<String, String> {
    // convert keyevent to string to compare with shortcut


    let mut pressed: Vec<String> = vec![];

    let modifier = key.modifiers.to_string();
    let char = key.code.to_string();

    if !char.is_empty() {
        pressed.push(char.to_lowercase());
    } else {
        return Err(String::from("Needs char"));
    }

    if !modifier.is_empty() {
        pressed.push(modifier.to_lowercase());
    }

    //println!("{:?}", pressed);

    for shortcut in mosaic.shortcut_handler.get_shortcuts() {
        let mut input: Vec<String> = shortcut.input.split("+").map(String::from).collect();
        println!("{:?}, {:?}", input, pressed);
        if input.sort() == pressed.sort() {
            return (shortcut.handler)(mosaic)
        }
    }

    //println!("{}", pressed);

    Ok(String::from("Key is unmapped"))
}

fn old_process_key(mosaic: &mut Mosaic, key: KeyEvent) {
    /*const PREFIX_TIMEOUT: Duration = Duration::from_millis(500);

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
    }*/

    // Fallback to normal mode-specific handling
    match mosaic.state_handler.mode {
        Mode::Normal => normal::handle_mode(mosaic, key),
        Mode::Insert => insert::handle_mode(mosaic, key),
        Mode::Command => command::handle_mode(mosaic, key),
    }
}

fn handle_non_modifier(mosaic: &mut Mosaic, key_event: KeyEvent) {
    if mosaic.panel_handler.get_current_editor_panel().is_none() {
        return;
    }

    let editor = &mut mosaic.panel_handler.get_current_editor_panel().unwrap().editor;

    match key_event.code {
        KeyCode::Esc => mosaic.state_handler.mode = Mode::Normal,
        KeyCode::Tab => editor.tab(),

        KeyCode::Char(c) => editor.input(c),

        KeyCode::Left => editor.move_cursor(CursorMove::Back),
        KeyCode::Up => editor.move_cursor(CursorMove::Up),
        KeyCode::Down => editor.move_cursor(CursorMove::Down),
        KeyCode::Right => editor.move_cursor(CursorMove::Forward),

        KeyCode::Enter => editor.input('\n'),

        KeyCode::Backspace => {
            editor.backspace();
        },
        _ => {}
    }
}