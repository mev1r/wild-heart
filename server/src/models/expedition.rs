use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Expedition {
    pub id: Uuid,
    pub participants: Vec<Uuid>,
    pub kind: ExpeditionKind,
    pub started_at: DateTime<Utc>,
    pub ended_at: Option<DateTime<Utc>>,
}

impl Expedition {
    pub fn new(participants: Vec<Uuid>, kind: ExpeditionKind) -> Self {
        Self {
            id: Uuid::new_v4(),
            participants,
            kind,
            started_at: Utc::now(),
            ended_at: None,
        }
    }
}

impl super::Model for Expedition {
    fn id(&self) -> Uuid {
        self.id
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ExpeditionKind {
    Hunt,
}