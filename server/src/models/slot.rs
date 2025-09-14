use crate::models::item::Item;
use serde::{Deserialize, Serialize};
use strum_macros::EnumIter;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Slot {
    pub id: Uuid,
    pub player_id: Uuid,
    pub index: u64,
    pub item: Option<Item>,
    pub kind: SlotKind,
}

impl Slot {
    pub fn new(player_id: Uuid, kind: SlotKind, index: u64) -> Self {
        Self {
            id: Uuid::new_v4(),
            player_id,
            kind,
            index,
            item: None,
        }
    }

    pub fn is_equipment_slot(&self) -> bool {
        matches!(self.kind,
            SlotKind::Weapon |
            SlotKind::Armor  |
            SlotKind::Helmet |
            SlotKind::Gloves |
            SlotKind::Boots  |
            SlotKind::Ring   |
            SlotKind::Necklace |
            SlotKind::Earring |
            SlotKind::Cloak    |
            SlotKind::Belt     |
            SlotKind::Shoulders|
            SlotKind::Pants    |
            SlotKind::Mask     |
            SlotKind::Pendant
        )
    }
}

impl super::Model for Slot {
    fn id(&self) -> Uuid {
        self.id
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, EnumIter)]
pub enum SlotKind {
    Inventory,
    Rune,
    Consumable,
    Compass,
    Hand,
    Weapon,
    Shoulders,
    Helmet,
    Mask,
    Cloak,
    Armor,
    Belt,
    Gloves,
    Pants,
    Boots,
    Ring,
    Earring,
    Necklace,
    Pendant,
    Ground,
}