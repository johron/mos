use crate::panel::traits::PanelData;

pub struct Panel {
    pub data: Box<dyn PanelData>,
    pub controller: Box<dyn PanelController>,
    pub view: Box<dyn PanelView<Backend = UiBackend>>,
}
