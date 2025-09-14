use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PlayerResource {
    pub id: Uuid,
    pub player_id: Uuid,
    pub hp: u64,
    pub max_hp: u64,
    pub mp: u64,
    pub max_mp: u64,
    pub energy: u64,
    pub max_energy: u64,
    pub weight_limit: u64,
}

impl PlayerResource {
    pub fn new(player_id: Uuid) -> Self {
        Self {
            id: Uuid::new_v4(),
            player_id,
            hp: 200,
            max_hp: 200,
            mp: 50,
            max_mp: 50,
            energy: 500,
            max_energy: 500,
            weight_limit: 100,
        }
    }
}

impl super::Model for PlayerResource {
    fn id(&self) -> Uuid {
        self.id
    }
}