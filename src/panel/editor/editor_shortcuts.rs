use crate::{Mode, Mosaic};
use crate::handler::config_handler::ConfigHandler;
use crate::handler::shortcut_handler::ShortcutHandler;

pub fn register_shortcuts(shortcut_handler: &mut ShortcutHandler, config_handler: &ConfigHandler) {
    let editor = &config_handler.config.editor;

    // Normal
    shortcut_handler.register(String::from("mode.normal.enter_insert_mode"), editor.normal_mode.shortcuts.enter_insert_mode.clone(), enter_insert_mode);
    shortcut_handler.register(String::from("mode.normal.enter_command_mode"), editor.normal_mode.shortcuts.enter_command_mode.clone(), enter_command_mode);

    // Command
    shortcut_handler.register(String::from("mode.command.enter_normal_mode"), editor.command_mode.shortcuts.enter_normal_mode.clone(), enter_normal_mode);

    // Insert
    shortcut_handler.register(String::from("mode.insert.enter_normal_mode"), editor.insert_mode.shortcuts.enter_normal_mode.clone(), enter_normal_mode);
    shortcut_handler.register(String::from("mode.insert.newline"), editor.insert_mode.shortcuts.newline.clone(), newline);
    shortcut_handler.register(String::from("mode.insert.backspace"), editor.insert_mode.shortcuts.backspace.clone(), backspace);
    shortcut_handler.register(String::from("mode.insert.tab"), editor.insert_mode.shortcuts.tab.clone(), tab);
    shortcut_handler.register(String::from("mode.insert.reverse_tab"), editor.insert_mode.shortcuts.reverse_tab.clone(), reverse_tab);

}

pub fn enter_normal_mode(mosaic: &mut Mosaic) -> Result<String, String> {
    mosaic.state_handler.mode = Mode::Normal;
    Ok(String::from("Entered normal mode"))
}

pub fn enter_insert_mode(mosaic: &mut Mosaic) -> Result<String, String> {
    mosaic.state_handler.mode = Mode::Insert;
    Ok(String::from("Entered normal mode"))
}

pub fn enter_command_mode(mosaic: &mut Mosaic) -> Result<String, String> {
    mosaic.state_handler.command.result = None;
    mosaic.state_handler.mode = Mode::Command;
    Ok(String::from("Entered command mode"))
}

pub fn newline(mosaic: &mut Mosaic) -> Result<String, String> {
    if mosaic.panel_handler.get_current_editor_panel().is_none() {
        return Err(String::from("No active editor"))
    }

    let editor = &mut mosaic.panel_handler.get_current_editor_panel().unwrap().editor;

    let current_top_line = editor.rope.get_line(editor.cursors[0].line).unwrap().to_string();
    let mut preceding_whitespace = String::new();
    for c in current_top_line.chars() {
        if !c.is_whitespace() {
            break
        }
        preceding_whitespace.push(c);
    }
    editor.input('\n');
    for c in preceding_whitespace.chars() {
        editor.input(c);
    }

    Ok(String::from("Newline"))
}


pub fn backspace(mosaic: &mut Mosaic) -> Result<String, String> {
    mosaic.panel_handler.get_current_editor_panel().unwrap().editor.backspace();
    Ok(String::from("Backspace"))
}

pub fn tab(mosaic: &mut Mosaic) -> Result<String, String> {
    mosaic.panel_handler.get_current_editor_panel().unwrap().editor.tab();
    Ok(String::from("Tab"))
}

pub fn reverse_tab(mosaic: &mut Mosaic) -> Result<String, String> {
    if mosaic.panel_handler.get_current_editor_panel().is_none() {
        return Err(String::from("No active editor"))
    }

    let editor = &mut mosaic.panel_handler.get_current_editor_panel().unwrap().editor;
    let tab_size = mosaic.config_handler.config.editor.tab_size;

    let current_top_line = editor.rope.get_line(editor.cursors[0].line).unwrap().to_string();
    let mut preceding_whitespace = String::new();
    for c in current_top_line.chars() {
        if !c.is_whitespace() {
            break
        }
        preceding_whitespace.push(c);
    }

    let mut to_remove = tab_size;
    if preceding_whitespace.len() < tab_size {
        to_remove = preceding_whitespace.len();
    }

    for _ in 0..to_remove {
        editor.backspace();
    }

    Ok(String::from("Reverse tab"))
}