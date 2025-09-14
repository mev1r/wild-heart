use crate::models::expedition::ExpeditionKind;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ItemStats {
    pub attack: Option<u64>,
    pub attack_speed: Option<u64>,
    pub hp_regeneration: Option<u64>,
    pub hp_regeneration_interval: Option<u64>,
    pub expedition_duration: Option<u64>,
    pub expedition_kind: Option<ExpeditionKind>,
}

impl ItemStats {
    pub fn new(
        attack: Option<u64>,
        attack_speed: Option<u64>,
        hp_regeneration: Option<u64>,
        hp_regeneration_interval: Option<u64>,
        expedition_duration: Option<u64>,
        expedition_kind: Option<ExpeditionKind>,
    ) -> Self {
        Self {
            attack,
            attack_speed,
            hp_regeneration,
            hp_regeneration_interval,
            expedition_duration,
            expedition_kind,
        }
    }
}