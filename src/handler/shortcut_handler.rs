use crate::Mosaic;

#[derive(Debug, Clone)]
pub struct Shortcut {
    pub name: String,
    pub input: String,
    pub handler: fn(&mut Mosaic) -> Result<String, String>,
}

#[derive(Debug, Clone)]
pub(crate) struct ShortcutHandler {
    shortcuts: Vec<Shortcut>,
}

impl ShortcutHandler {
    pub fn new() -> Self {
        Self {
            shortcuts: Vec::new(),
        }
    }

    pub fn register(&mut self, name: String, input: String, handler: fn(&mut Mosaic) -> Result<String, String>) {
        let shortcut = Shortcut {
            name,
            input,
            handler,
        };

        self.shortcuts.push(shortcut);
    }

    pub fn get_shortcuts(&self) -> &Vec<Shortcut> {
        &self.shortcuts
    }
}