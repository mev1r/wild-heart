use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Expedition {
    pub id: Uuid,
    pub participant_id: Uuid,
    pub kind: ExpeditionKind,
    pub started_at: DateTime<Utc>,
    pub ended_at: DateTime<Utc>,
}

impl Expedition {
    pub fn new(participant_id: Uuid, kind: ExpeditionKind, ended_at: DateTime<Utc>) -> Self {
        Self {
            id: Uuid::new_v4(),
            participant_id,
            kind,
            started_at: Utc::now(),
            ended_at,
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