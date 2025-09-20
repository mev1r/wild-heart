mod incoming;
mod outgoing;

pub use incoming::*;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct IncomingMessage<T> {
    pub event: IncomingEvent,
    pub data: Option<T>,
}

#[derive(Debug, Serialize)]
pub struct OutgoingMessage<T> {
    pub event: OutgoingEvent,
    pub data: T,
    pub id: Uuid,
}

impl<T> OutgoingMessage<T> {
    pub fn new(event: OutgoingEvent, data: T) -> Self {
        Self {
            event,
            data,
            id: Uuid::new_v4(),
        }
    }
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
pub enum IncomingEvent {
    TakeItem,
    DropItem,
    SendChatMessage,
    StartExpedition,
    EndExpedition,
    ToggleLoot,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum OutgoingEvent {
    PlayerInfo,
    PlayerResource,
    PlayerAttributes,
    PlayerState,
    PlayerStats,
    Slots,
    Meta,
    ChatMessage,
    ExpeditionCountup,
    Log,
}