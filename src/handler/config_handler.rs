use std::time::Duration;
use crate::{ui, Mosaic};
use crate::handler::command_handler::{Command, CommandHandler};

#[derive(Debug)]
pub(crate) struct ConfigHandler {
    command_handler: CommandHandler,
}

impl ConfigHandler {
    pub fn new(command_handler: CommandHandler) -> Self {
        Self {
            command_handler,
        }
    }

    pub fn load_config(&mut self) {

    }

    fn register_commands(&mut self) { // will probably remove this function
        self.command_handler.add_command_space("mos");

        self.command_handler.register_command(Command {
            name: String::from("reload"),
            handler: Self::reload
        }, "mos")

        // replace/refactor with the stuff from configs, and plugins...
    }

    fn reload(mosaic: &mut Mosaic, args: Vec<&str>) -> Result<String, String> {
        mosaic.show_toast(&args.join(" "), Duration::from_secs(3));

        Ok(String::from("Very nice"))
    }
}