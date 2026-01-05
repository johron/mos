use crate::{Command, Mode};
use crate::handler::panel_handler::ExtendDirection;

#[derive(Clone, Debug)]
pub struct StateHandler {
    pub should_quit: bool,
    pub mode: Mode,
    pub command: Command,
    pub extend_direction: ExtendDirection
}

impl StateHandler {
    pub fn new() -> Self {
        Self {
            should_quit: false,
            mode: Mode::Normal,
            command: Command::new(),
            extend_direction: ExtendDirection::Horizontal,
        }
    }
}