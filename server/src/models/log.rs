use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Log {
    pub id: Uuid,
    pub message: String,
    pub timestamp: DateTime<Utc>,
}

impl Log {
    pub fn new(message: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            message,
            timestamp: Utc::now(),
        }
    }
}

impl super::Model for Log {
    fn id(&self) -> Uuid {
        self.id
    }
}