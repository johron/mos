use crate::Mosaic;

#[derive(Debug)]
struct CommandSpace {
    name: String,
    commands: Vec<Command>,
}

#[derive(Debug)]
pub(crate) struct Command {
    pub(crate) name: String,
    pub(crate) handler: fn(&mut Mosaic, Vec<&str>) -> Result<String, String>,
}

#[derive(Debug)]
pub(crate) struct CommandHandler {
    spaces: Vec<CommandSpace>,
}

impl CommandHandler {
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

    pub fn register_command(&mut self, command: Command, namespace: &str) {
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
    
    pub fn handle_command(&self, namespace: &str, command_name: &str, mosaic: &mut Mosaic, args: Vec<&str>) -> Option<Result<String, String>> {
        if let Some(space) = self.spaces.iter().find(|space| space.name == namespace) {
            if let Some(command) = space.commands.iter().find(|cmd| cmd.name == command_name) {
                return Some((command.handler)(mosaic, args));
            }
        }
        None
    }
}