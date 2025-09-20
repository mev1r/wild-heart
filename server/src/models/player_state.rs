use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PlayerState {
    pub id: Uuid,
    pub player_id: Uuid,
    pub in_combat: bool,
    pub is_attacking: bool,
    pub is_looting: bool,
    pub target_id: Option<Uuid>,
}

impl PlayerState {
    pub fn new(player_id: Uuid) -> Self {
        Self {
            id: Uuid::new_v4(),
            player_id,
            in_combat: false,
            is_attacking: false,
            is_looting: false,
            target_id: None,
        }
    }
}

impl super::Model for PlayerState {
    fn id(&self) -> Uuid {
        self.id
    }
}