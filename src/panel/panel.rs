use uuid::Uuid;
use crate::panel::traits::{PanelController};

#[derive(Clone, Copy, Hash, Eq, PartialEq)]
pub struct PanelId(Uuid);

pub struct Panel {
    pub id: PanelId,
    pub title: String,
    pub controller: Box<dyn PanelController>,
}
