use crate::{Command, Mode, Mosaic};
use crossterm::event::{self, Event, KeyCode, KeyEvent};
use std::io::Error;


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
        let mods = modifier.split("+");
        for modi in mods {
            pressed.push(modi.to_lowercase());
        }
    }

    pressed.sort();

    for shortcut in mosaic.shortcut_handler.get_shortcuts() {
        let mode = format!("editor.{}", mosaic.state_handler.mode.clone().to_string().to_lowercase());

        if shortcut.name.starts_with(mode.as_str()) {
            let mut input: Vec<String> = shortcut.input.split("+").map(String::from).collect();
            input.sort();
            if input == pressed {
                return (shortcut.handler)(mosaic, pressed);
            }
        }
    }

    match mosaic.state_handler.mode {
        Mode::Insert => handle_input_mode(mosaic, key),
        Mode::Command =>  handle_command_mode(mosaic, key),
        _ => {
            Ok(String::from("Input is unmapped"))
        }
    }
}

fn handle_input_mode(mosaic: &mut Mosaic, key_event: KeyEvent) -> Result<String, String> {
    if mosaic.panel_handler.get_current_editor_panel().is_none() {
        return Err(String::from("No active editor"))
    }

    let editor = &mut mosaic.panel_handler.get_current_editor_panel().unwrap().editor;

    match key_event.code {
        KeyCode::Esc => mosaic.state_handler.mode = Mode::Normal,
        KeyCode::Tab => editor.tab(),

        KeyCode::Char(c) => editor.input(c),
        KeyCode::Enter => editor.input('\n'),

        KeyCode::Backspace => {
            editor.backspace();
        },
        _ => {
            return Ok(String::from("Unmapped input"));
        }
    }

    Ok(String::from("Inputted"))
}

pub fn handle_command_mode(mosaic: &mut Mosaic, key: KeyEvent) -> Result<String, String> {
    match key.code {
        KeyCode::Esc => {
            mosaic.state_handler.command.result = None;
            mosaic.state_handler.mode = Mode::Normal;
        },
        KeyCode::Enter => {
            let res = handle_command(mosaic);

            mosaic.state_handler.command = Command {
                content: String::new(),
                result: Some(res.unwrap_or_else(|e| format!("Error: {}", e))),
            };

            mosaic.state_handler.mode = Mode::Normal;
        },
        KeyCode::Char(c) => {
            mosaic.state_handler.command += c.to_string().as_str();
        },
        KeyCode::Backspace => {
            mosaic.state_handler.command.pop();
        },
        _ => {
            return Ok(String::from("Unmapped input"));
        }
    }

    Ok(String::from("Inputted command"))
}

pub fn handle_command(mosaic: &mut Mosaic) -> Result<String, String> {
    let args = mosaic.state_handler.command.content.split_whitespace().map(|s| s.to_string()).collect::<Vec<_>>();

    let commands = mosaic.command_handler.get_commands("@");

    if args.is_empty() || args[0].is_empty() {
        return Err(String::from("No command provided"));
    }

    if let Some(cmds) = commands {
        if let Some(command) = cmds.iter().find(|cmd| cmd.name == args[0]) {
            (command.handler)(mosaic, args)
        } else {
            Err(format!("Unknown command: {}", args[0]))
        }
    } else {
        Err(String::from("No command namespace found"))
    }
}