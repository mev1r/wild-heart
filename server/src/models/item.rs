use crate::models::item_stats::ItemStats;
use crate::models::{Slot, SlotKind};
use crate::server::GameServer;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Item {
    pub id: Uuid,
    pub player_id: Uuid,
    pub kind: ItemKind,
    pub name: String,
    pub tier: ItemTier,
    pub icon: String,
    pub quantity: u64,
    pub level: u32,
    pub enchanted: u32,
    pub description: String,
    pub weight: f32,
    pub is_stackable: bool,
    pub is_usable: bool,
    pub stats: Option<ItemStats>,
}

impl Item {
    pub fn new(
        player_id: Uuid,
        kind: ItemKind,
        name: &str,
        tier: ItemTier,
        icon: &str,
        quantity: u64,
        level: u32,
        enchanted: u32,
        description: &str,
        weight: f32,
        is_stackable: bool,
        is_usable: bool,
        stats: Option<ItemStats>,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            player_id,
            kind,
            name: name.to_string(),
            icon: icon.to_string(),
            tier,
            quantity,
            level,
            enchanted,
            description: description.to_string(),
            weight,
            is_stackable,
            is_usable,
            stats,
        }
    }

    pub fn add_to_empty_slot(&self, kind: SlotKind) -> Result<(), String> {
        let server = GameServer::global();

        let mut slots: Vec<Slot> = server.slots_store
            .find_all_by(|slot| {
                slot.player_id == self.player_id && slot.kind == kind
            });

        slots.sort_by_key(|slot| slot.index);

        let empty_slot = slots
            .into_iter()
            .find(|slot| slot.item.is_none())
            .ok_or("No empty slot found")?;

        server.slots_store.update(&empty_slot.id, |slot| {
            slot.item = Some(self.clone());
        })?;

        Ok(())
    }
}

impl super::Model for Item {
    fn id(&self) -> Uuid {
        self.id
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ItemKind {
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
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ItemTier {
    Common,
    Uncommon,
    Rare,
    Epic,
    Legendary,
}