use std::io::Error;
use crossterm::event::{KeyCode, KeyEvent};
use crate::{Command, Mode, Mosaic};

pub fn handle_mode(mosaic: &mut Mosaic, key: KeyEvent) {
    match key.code {
        KeyCode::Esc => {
            mosaic.command.result = None;
            mosaic.set_mode(Mode::Normal)
        },
        KeyCode::Enter => {
            let res = handle_command(mosaic);

            mosaic.command = Command {
                content: String::new(),
                result: Some(res.unwrap_or_else(|e| format!("Error: {}", e))),
            };

            mosaic.set_mode(Mode::Normal);
        },
        KeyCode::Char(c) => {
            mosaic.command += c.to_string().as_str();
        },
        KeyCode::Backspace => {
            mosaic.command.pop();
        },
        _ => {}
    }
}

pub(crate) fn handle_command(mosaic: &mut Mosaic) -> Result<String, Error> {
    let editor = &mut mosaic.editors[mosaic.current_editor];
    let args = mosaic.command.content.as_str().split(' ').collect::<Vec<_>>();

    match args[0] {
        "q" => {
            let current_content = editor.rope.to_string();

            if let Some(path) = mosaic.editors[mosaic.current_editor].file_path.as_ref() {
                match std::fs::read_to_string(path) {
                    Ok(disk) => {
                        if disk != current_content {
                            return Err(Error::new(std::io::ErrorKind::Other, "Unsaved changes present"));
                        }
                    }
                    Err(_) => {
                        if !current_content.is_empty() {
                            return Err(Error::new(std::io::ErrorKind::Other, "Unsaved changes or unreadable file"));
                        }
                    }
                }
            } else {
                if !current_content.is_empty() {
                    return Err(Error::new(std::io::ErrorKind::Other, "Unsaved changes (no file path)"));
                }
            }

            mosaic.quit();
            Ok(String::from("Quit command executed"))
        },
        "q!" => {
            mosaic.quit();
            Ok(String::from("Force quit command executed"))
        },
        "w" => {
            let content = editor.rope.to_string();

            if mosaic.editors[mosaic.current_editor].file_path.is_none() {
                if args.len() < 2 {
                    return Err(Error::new(std::io::ErrorKind::Other, "No file path provided"));
                } else {
                    mosaic.editors[mosaic.current_editor].file_path = Some(args[1].to_string());
                }
            }

            let file_path = mosaic.editors[mosaic.current_editor]
                .file_path
                .as_ref()
                .unwrap();

            std::fs::write(file_path, content.as_bytes())?;

            Ok(format!("Wrote {} bytes to file", content.len()))
        },
        "f" => {
            let search_term = "test";
            //editor.set_search_pattern(search_term).unwrap();
            //editor.search_forward(true);

            Ok(format!("Search pattern set to '{}'", search_term))
        },
        _ => {
            Ok(String::from("Error: Unknown command"))
        }
    }
}