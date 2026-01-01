use crate::Mosaic;

#[derive(Debug, Clone)]
pub struct CommandSpace {
    name: String,
    commands: Vec<Command>,
}

#[derive(Debug, Clone)]
pub(crate) struct Command {
    pub(crate) name: String,
    pub(crate) handler: fn(&mut Mosaic, Vec<String>) -> Result<String, String>,
}

#[derive(Debug, Clone)]
pub(crate) struct CommandHandler {
    pub(crate) spaces: Vec<CommandSpace>,
}

impl CommandHandler { // TODO: remove command spaces just have string and use cmd.cmd.cmd...
    pub fn new() -> Self {
        Self {
            spaces: Vec::from([
                CommandSpace {
                    name: String::from("@"), // Global namespace, (w, q, etc.)
                    commands: Vec::new(),
                },
            ]),
        }
    }
    
    pub fn get_commands(&self, namespace: &str) -> Option<&Vec<Command>> {
        self.spaces.iter()
            .find(|space| space.name == namespace)
            .map(|space| &space.commands)
    }
    
    pub fn register_command_space(&mut self, namespace: &str) {
        if !self.spaces.iter().any(|space| space.name == namespace) {
            let new_space = CommandSpace {
                name: namespace.to_string(),
                commands: Vec::new(),
            };
            self.spaces.push(new_space);
        }
    }

    pub fn register(&mut self, name: String, namespace: &str, handler: fn(&mut Mosaic, Vec<String>) -> Result<String, String>) {
        let command = Command {
            name,
            handler,
        };

        if let Some(space) = self.spaces.iter_mut().find(|space| space.name == namespace) {
            space.commands.push(command);
        } else {
            let new_space = CommandSpace {
                name: namespace.to_string(),
                commands: vec![command],
            };
            self.spaces.push(new_space);
        }
    }
}