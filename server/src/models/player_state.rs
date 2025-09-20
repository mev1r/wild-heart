use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PlayerState {
    pub id: Uuid,
    pub player_id: Uuid,
    pub is_looting: bool,
}

impl PlayerState {
    pub fn new(player_id: Uuid) -> Self {
        Self {
            id: Uuid::new_v4(),
            player_id,
            is_looting: false,
        }
    }
}

impl super::Model for PlayerState {
    fn id(&self) -> Uuid {
        self.id
    }
}