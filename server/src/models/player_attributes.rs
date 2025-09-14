use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PlayerAttributes {
    pub id: Uuid,
    pub player_id: Uuid,
    pub strength: u32,
    pub dexterity: u32,
    pub vitality: u32,
    pub intelligence: u32,
    pub spirit: u32,
    pub luck: u32,
}

impl PlayerAttributes {
    pub fn new(player_id: Uuid, strength: u32, dexterity: u32, vitality: u32, intelligence: u32, spirit: u32, luck: u32) -> Self {
        Self {
            id: Uuid::new_v4(),
            player_id,
            strength,
            dexterity,
            vitality,
            intelligence,
            spirit,
            luck,
        }
    }
}

impl super::Model for PlayerAttributes {
    fn id(&self) -> Uuid {
        self.id
    }
}