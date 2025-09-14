use crate::models::{ChatKind, SlotKind};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct TakeItem {
    pub index: u64,
    pub kind: SlotKind,
}

#[derive(Debug, Deserialize)]
pub struct DropItem {
    pub index: u64,
    pub kind: SlotKind,
}

#[derive(Debug, Deserialize)]
pub struct SendChatMessage {
    pub kind: ChatKind,
    pub recipient: Option<String>,
    pub content: String,
}

#[derive(Debug, Deserialize)]
pub struct StartExpedition {
    // No payload is required to start an expedition
}
