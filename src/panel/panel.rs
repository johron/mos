use ratatui::Frame;
use ratatui::layout::Rect;
use uuid::Uuid;
use crate::app::MosId;
use crate::event::event::Event;

pub type PanelCtor = fn() -> Box<dyn Panel>;

pub trait Panel {
    fn id(&self) -> MosId;
    fn title(&self) -> &str;
    
    // fn plugin_id(&self) -> MosId; // maybe panels should also have a reference to their plugin
    // fn kind(&self) -> MosId; // kind?
    
    // fn in_normal() -> bool; // if is in normal mode, panels only get input in normal mode, this should probably be an an event, reveresed not managed by the panel, defined in workspace maybe?
    
    fn handle_event(&mut self, event: Event);
    fn render(&self, frame: &mut Frame, area: Rect);
}