use std::any::Any;
use uuid::Uuid;

pub trait PanelData: Send + Sync {
    fn id(&self) -> Uuid;
    fn title(&self) -> String;
    fn serialize(&self) -> serde_json::Value;
    fn deserialize(&mut self, data: serde_json::Value);
    fn as_any(&self) -> &dyn Any;
}
