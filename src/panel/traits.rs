use std::any::Any;
use ratatui::{Frame, layout::Rect};
use crate::input::event::InputEvent;

pub trait PanelController {
    fn handle_input(
        &mut self,
        input: &InputEvent,
    );

    fn update(&mut self);

    fn render(
        &mut self,
        frame: &mut Frame,
        area: Rect,
    );
}
