use serde::{Deserialize, Serialize};

pub const BASE_ATTACK_SPEED: u64 = 3000;
pub const BASE_HP_REGENERATION: u64 = 1;
pub const BASE_HP_REGENERATION_INTERVAL: u64 = 2000;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BaseStats {
    pub base_attack_speed: u64,
    pub base_hp_regeneration: u64,
    pub base_hp_regen_interval: u64,
}

impl BaseStats {
    pub fn new() -> Self {
        Self {
            base_attack_speed: BASE_ATTACK_SPEED,
            base_hp_regeneration: BASE_HP_REGENERATION,
            base_hp_regen_interval: BASE_HP_REGENERATION_INTERVAL,
        }
    }
}