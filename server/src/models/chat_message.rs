use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ChatMessage {
    pub id: Uuid,
    pub sender: String,
    pub recipient: Option<String>,
    pub kind: ChatKind,
    pub content: String,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ChatKind {
    General,
    Trade,
    Whisper,
}

impl ChatMessage {
    pub fn new(sender: String, recipient: Option<String>, kind: ChatKind, content: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            sender,
            recipient,
            kind,
            content,
            timestamp: Utc::now(),
        }
    }

    pub fn general(sender: String, content: String) -> Self {
        Self::new(sender, None, ChatKind::General, content)
    }

    pub fn trade(sender: String, content: String) -> Self {
        Self::new(sender, None, ChatKind::Trade, content)
    }

    pub fn whisper(sender: String, recipient: String, content: String) -> Self {
        Self::new(sender, Some(recipient), ChatKind::Whisper, content)
    }
}

impl super::Model for ChatMessage {
    fn id(&self) -> Uuid {
        self.id
    }
}
