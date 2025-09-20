use crate::messages::{OutgoingEvent, OutgoingMessage};
use crate::meta::{BASE_ATTACK_SPEED, BASE_HP_REGENERATION, BASE_HP_REGENERATION_INTERVAL};
use crate::models::PlayerAttributes;
use crate::server::{GameServer, WebSocketManager};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PlayerStats {
    pub id: Uuid,
    pub player_id: Uuid,
    pub attack: u64,
    pub attack_speed: u64,
    pub energy_regeneration: u64,
    pub energy_regeneration_interval: u64,
}

impl PlayerStats {
    pub fn new(player_id: Uuid) -> Self {
        let server = GameServer::global();

        let attributes = server.player_attributes_store
            .find_by(|attr| attr.player_id == player_id).unwrap();

        Self {
            id: Uuid::new_v4(),
            player_id,
            attack: Self::calculate_attack(&attributes),
            attack_speed: Self::calculate_attack_speed(&attributes),
            energy_regeneration: Self::calculate_energy_regeneration(&attributes),
            energy_regeneration_interval: Self::calculate_energy_regeneration_interval(&attributes),
        }
    }

    fn calculate_attack(attributes: &PlayerAttributes) -> u64 {
        let base_attack = 20;
        let strength_bonus = attributes.strength as u64 * 5;
        let dex_bonus = attributes.dexterity as u64 * 2;

        let flat_attack = base_attack + strength_bonus + dex_bonus;

        let strength_percent_bonus = (flat_attack * attributes.strength as u64) / 100;

        let base_total = flat_attack + strength_percent_bonus;

        let equipment_attack = Self::get_equipment_stat(attributes.player_id, |stats| stats.attack.unwrap_or(0));

        base_total + equipment_attack
    }

    fn calculate_attack_speed(attributes: &PlayerAttributes) -> u64 {
        let base_attack_speed = BASE_ATTACK_SPEED;
        let dex_reduction = attributes.dexterity as u64 * 10;
        let strength_penalty = attributes.strength as u64 * 5;

        let calculated_speed = base_attack_speed + strength_penalty - dex_reduction;

        let base_speed = calculated_speed.max(500);

        let equipment_speed_modifier = Self::get_equipment_stat(attributes.player_id, |stats| stats.attack_speed.unwrap_or(0) as u64);

        base_speed - equipment_speed_modifier
    }

    fn calculate_energy_regeneration(attributes: &PlayerAttributes) -> u64 {
        let base_hp_regen = BASE_HP_REGENERATION;
        let vitality_bonus = attributes.vitality as u64 / 10;
        let spirit_bonus = attributes.spirit as u64 / 15;

        let base_total = base_hp_regen + vitality_bonus + spirit_bonus;

        let equipment_regen = Self::get_equipment_stat(attributes.player_id, |stats| stats.energy_regeneration.unwrap_or(0));

        base_total + equipment_regen
    }

    fn calculate_energy_regeneration_interval(attributes: &PlayerAttributes) -> u64 {
        let base_interval = BASE_HP_REGENERATION_INTERVAL;
        let vitality_reduction = attributes.vitality as u64 * 1;
        let spirit_reduction = attributes.spirit as u64 * 1;

        let calculated_interval = base_interval - vitality_reduction - spirit_reduction;
        let base_interval = calculated_interval.max(300);

        let equipment_interval_modifier = Self::get_equipment_stat(attributes.player_id, |stats| stats.energy_regeneration_interval.unwrap_or(0));

        (base_interval - equipment_interval_modifier).max(300)
    }

    pub fn recalculate(&self) -> Result<PlayerStats, String> {
        let server = GameServer::global();

        let attributes = server.player_attributes_store
            .find_by(|attr| attr.player_id == self.player_id)
            .ok_or("Player attributes not found")?;

        let updated = server.player_stats_store.update(&self.id, |stats| {
            stats.attack = Self::calculate_attack(&attributes);
            stats.attack_speed = Self::calculate_attack_speed(&attributes);
            stats.energy_regeneration = Self::calculate_energy_regeneration(&attributes);
            stats.energy_regeneration_interval = Self::calculate_energy_regeneration_interval(&attributes);
        });

        let player_id = self.player_id;
        let result = updated.clone();

        tokio::spawn(async move {
            let data = updated.clone();
            if let Ok(stats) = data {
                let ws = WebSocketManager::global();
                let msg = OutgoingMessage::new(
                    OutgoingEvent::PlayerStats,
                    Box::new(stats) as Box<dyn erased_serde::Serialize + Send>,
                );
                ws.send_to_player(player_id, msg).await;
            }
        });

        result
    }

    fn get_equipment_stat<F>(player_id: Uuid, stat_extractor: F) -> u64
    where
        F: Fn(&crate::models::ItemStats) -> u64,
    {
        let server = GameServer::global();

        let equipment_slots = server.slots_store.find_all_by(|slot| {
            slot.player_id == player_id && matches!(slot.kind,
            crate::models::SlotKind::Weapon |
            crate::models::SlotKind::Armor |
            crate::models::SlotKind::Helmet |
            crate::models::SlotKind::Gloves |
            crate::models::SlotKind::Boots |
            crate::models::SlotKind::Ring |
            crate::models::SlotKind::Necklace |
            crate::models::SlotKind::Earring |
            crate::models::SlotKind::Cloak |
            crate::models::SlotKind::Belt |
            crate::models::SlotKind::Shoulders |
            crate::models::SlotKind::Pants |
            crate::models::SlotKind::Mask |
            crate::models::SlotKind::Pendant
        )
        });

        let mut total_stat = 0;

        for slot in equipment_slots {
            if let Some(item) = &slot.item {
                if let Some(stats) = &item.stats {
                    total_stat += stat_extractor(stats);
                }
            }
        }

        total_stat
    }
}

impl super::Model for PlayerStats {
    fn id(&self) -> Uuid {
        self.id
    }
}