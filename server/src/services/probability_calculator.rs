use crate::meta::level_to_exp;
use crate::models::{Item, Player, PlayerAttributes, PlayerStats, Slot};
use crate::server::GameServer;
use rand::Rng;
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub enum ProbabilityType {
    ExpGain,
    LootDrop,
    CinGain,
}

#[derive(Debug, Clone)]
pub enum FrequencyType {
    ExpRoll,
    LootRoll,
    CinRoll,
}

pub struct ProbabilityCalculator;

impl ProbabilityCalculator {
    pub fn calculate(player_id: Uuid, prob_type: ProbabilityType) -> Result<f32, String> {
        let context = Self::gather_player_context(player_id)?;

        match prob_type {
            ProbabilityType::ExpGain => Self::calculate_exp_gain(&context),
            ProbabilityType::LootDrop => Self::calculate_loot_drop(&context),
            ProbabilityType::CinGain => Self::calculate_cin_gain(&context),
        }
    }

    pub fn calculate_frequency(player_id: Uuid, freq_type: FrequencyType) -> Result<u64, String> {
        let context = Self::gather_player_context(player_id)?;

        match freq_type {
            FrequencyType::ExpRoll => Self::calculate_exp_frequency(&context),
            FrequencyType::LootRoll => Self::calculate_loot_frequency(&context),
            FrequencyType::CinRoll => Self::calculate_cin_frequency(&context),
        }
    }

    pub fn exp_gain_chance(player_id: Uuid) -> f32 {
        Self::calculate(player_id, ProbabilityType::ExpGain).unwrap_or(0.1)
    }

    pub fn loot_drop_chance(player_id: Uuid) -> f32 {
        Self::calculate(player_id, ProbabilityType::LootDrop).unwrap_or(0.05)
    }

    pub fn exp_roll_frequency(player_id: Uuid) -> u64 {
        Self::calculate_frequency(player_id, FrequencyType::ExpRoll).unwrap_or(10)
    }

    pub fn loot_roll_frequency(player_id: Uuid) -> u64 {
        Self::calculate_frequency(player_id, FrequencyType::LootRoll).unwrap_or(15)
    }

    pub fn calculate_exp_amount(player_id: Uuid) -> u64 {
        Self::get_exp_amount(player_id).unwrap_or(8)
    }

    pub fn cin_gain_chance(player_id: Uuid) -> f32 {
        Self::calculate(player_id, ProbabilityType::CinGain).unwrap_or(0.12)
    }

    pub fn cin_roll_frequency(player_id: Uuid) -> u64 {
        Self::calculate_frequency(player_id, FrequencyType::CinRoll).unwrap_or(20)
    }

    pub fn calculate_cin_amount(player_id: Uuid) -> u64 {
        Self::get_cin_amount(player_id).unwrap_or(5)
    }

    fn gather_player_context(player_id: Uuid) -> Result<PlayerContext, String> {
        let server = GameServer::global();

        let player = server.player_store
            .find_by(|p| p.id == player_id)
            .ok_or("Player not found")?;

        let attributes = server.player_attributes_store
            .find_by(|attr| attr.player_id == player_id)
            .ok_or("Player attributes not found")?;

        let stats = server.player_stats_store
            .find_by(|s| s.player_id == player_id)
            .ok_or("Player stats not found")?;

        let equipment = server.slots_store.find_all_by(|slot| {
            slot.player_id == player_id && slot.is_equipment_slot() && slot.item.is_some()
        });

        let level = Self::calculate_level(player.exp);

        let compass = server.slots_store
            .find_by(|slot| {
                slot.player_id == player_id
                    && slot.kind == crate::models::SlotKind::Compass
                    && slot.item.is_some()
            })
            .and_then(|slot| slot.item.clone());

        Ok(PlayerContext {
            player,
            attributes,
            stats,
            equipment,
            level,
            compass,
        })
    }

    fn calculate_level(exp: u64) -> u8 {
        let level_map = level_to_exp();

        for (level, required_exp) in level_map.iter() {
            if exp < *required_exp {
                return level.saturating_sub(1).max(1);
            }
        }

        level_map.keys().max().copied().unwrap_or(1)
    }

    fn calculate_weighted_probability(
        context: &PlayerContext,
        base_chance: f32,
        weights: &HashMap<&str, f32>,
        equipment_modifier: f32,
    ) -> Result<f32, String> {
        let attr_score =
            (context.attributes.strength as f32 * weights.get("strength").unwrap_or(&1.0)) +
                (context.attributes.dexterity as f32 * weights.get("dexterity").unwrap_or(&1.0)) +
                (context.attributes.vitality as f32 * weights.get("vitality").unwrap_or(&1.0)) +
                (context.attributes.intelligence as f32 * weights.get("intelligence").unwrap_or(&1.0)) +
                (context.attributes.spirit as f32 * weights.get("spirit").unwrap_or(&1.0)) +
                (context.attributes.luck as f32 * weights.get("luck").unwrap_or(&1.0));

        let equipment_bonus = Self::calculate_equipment_bonus(&context.equipment, equipment_modifier);

        let level_bonus = (context.level as f32 - 1.0) * 0.005;

        let compass_penalty = if let Some(compass) = &context.compass && compass.enchanted > 0 && compass.level > 1 {
            let level_penalty = compass.level as f32 * 0.2;
            let enchant_penalty = compass.enchanted as f32 * 0.1;
            level_penalty + enchant_penalty
        } else {
            0.0
        };

        let final_probability = base_chance + (attr_score * 0.0001) + equipment_bonus + level_bonus - compass_penalty;

        Ok(final_probability.max(0.0001).min(1.0))
    }

    fn calculate_weighted_frequency(
        context: &PlayerContext,
        base_interval: u64,
        weights: &HashMap<&str, f32>,
        equipment_modifier: f32,
    ) -> Result<u64, String> {
        let attr_score =
            (context.attributes.strength as f32 * weights.get("strength").unwrap_or(&1.0)) +
                (context.attributes.dexterity as f32 * weights.get("dexterity").unwrap_or(&1.0)) +
                (context.attributes.vitality as f32 * weights.get("vitality").unwrap_or(&1.0)) +
                (context.attributes.intelligence as f32 * weights.get("intelligence").unwrap_or(&1.0)) +
                (context.attributes.spirit as f32 * weights.get("spirit").unwrap_or(&1.0)) +
                (context.attributes.luck as f32 * weights.get("luck").unwrap_or(&1.0));

        let equipment_bonus = Self::calculate_equipment_bonus(&context.equipment, equipment_modifier);

        let level_bonus = (context.level as f32 - 1.0);

        let compass_penalty = if let Some(compass) = &context.compass && compass.enchanted > 0 && compass.level > 1 {
            let level_penalty = compass.level as f32;
            let enchant_penalty = compass.enchanted as f32 * 0.9;
            level_penalty + enchant_penalty
        } else {
            0.0
        };

        let reduction = (attr_score * 0.05) + (equipment_bonus * 100.0) + level_bonus;
        let final_interval = (base_interval as f32 - reduction + compass_penalty).max(5.0);

        Ok(final_interval as u64)
    }

    fn calculate_equipment_bonus(equipment: &[Slot], modifier: f32) -> f32 {
        let mut bonus = 0.0;

        for slot in equipment {
            if let Some(item) = &slot.item {
                let tier_bonus = match item.tier {
                    crate::models::ItemTier::Common => 0.0,
                    crate::models::ItemTier::Uncommon => 0.01,
                    crate::models::ItemTier::Rare => 0.02,
                    crate::models::ItemTier::Epic => 0.035,
                    crate::models::ItemTier::Legendary => 0.05,
                };

                let level_bonus = item.level as f32 * 0.002;

                let enchant_bonus = item.enchanted as f32 * 0.005;

                let stats_bonus = if let Some(stats) = &item.stats {
                    let mut stat_contribution = 0.0;

                    if let Some(attack) = stats.attack {
                        stat_contribution += attack as f32 * 0.0001;
                    }

                    if let Some(defense) = stats.defense {
                        stat_contribution += defense as f32 * 0.0001;
                    }

                    stat_contribution
                } else {
                    0.0
                };

                bonus += (tier_bonus + level_bonus + enchant_bonus + stats_bonus) * modifier;
            }
        }

        bonus
    }

    fn calculate_exp_gain(context: &PlayerContext) -> Result<f32, String> {
        let weights = HashMap::from([
            ("strength", 1.2),
            ("dexterity", 1.1),
            ("vitality", 0.8),
            ("intelligence", 1.8),
            ("spirit", 1.4),
            ("luck", 2.2),
        ]);

        Self::calculate_weighted_probability(context, 0.1, &weights, 1.0)
    }

    fn calculate_exp_frequency(context: &PlayerContext) -> Result<u64, String> {
        let weights = HashMap::from([
            ("strength", 1.0),
            ("dexterity", 1.2),
            ("vitality", 0.8),
            ("intelligence", 1.5),
            ("spirit", 1.3),
            ("luck", 0.9),
        ]);

        Self::calculate_weighted_frequency(context, 18, &weights, 0.8)
    }

    fn get_exp_amount(player_id: Uuid) -> Result<u64, String> {
        let server = GameServer::global();

        let compass_slot = server.slots_store
            .find_by(|slot| {
                slot.player_id == player_id
                    && slot.kind == crate::models::SlotKind::Compass
                    && slot.item.is_some()
            })
            .ok_or("No compass equipped")?;

        let compass = compass_slot.item.as_ref().unwrap();

        let base_exp = 8u64;

        let level_bonus = compass.level as u64 * 3;

        let enchant_bonus = compass.enchanted as u64 * 2;

        let total_exp = base_exp + level_bonus + enchant_bonus;

        let mut rng = rand::thread_rng();
        let variance_factor = rng.gen_range(0.3..=1.1);
        let final_exp = (total_exp as f64 * variance_factor).round() as u64;

        Ok(final_exp)
    }

    fn calculate_loot_drop(context: &PlayerContext) -> Result<f32, String> {
        let weights = HashMap::from([
            ("strength", 0.9),
            ("dexterity", 1.6),
            ("vitality", 0.7),
            ("intelligence", 1.2),
            ("spirit", 0.9),
            ("luck", 3.0),
        ]);

        Self::calculate_weighted_probability(context, 0.04, &weights, 1.2)
    }

    fn calculate_loot_frequency(context: &PlayerContext) -> Result<u64, String> {
        let weights = HashMap::from([
            ("strength", 1.1),
            ("dexterity", 1.8),
            ("vitality", 1.0),
            ("intelligence", 1.4),
            ("spirit", 1.1),
            ("luck", 0.7),
        ]);

        Self::calculate_weighted_frequency(context, 24, &weights, 1.0)
    }

    fn calculate_cin_gain(context: &PlayerContext) -> Result<f32, String> {
        let weights = HashMap::from([
            ("strength", 1.0),
            ("dexterity", 1.3),
            ("vitality", 0.8),
            ("intelligence", 1.4),
            ("spirit", 1.0),
            ("luck", 2.5),
        ]);

        Self::calculate_weighted_probability(context, 0.12, &weights, 1.1)
    }

    fn calculate_cin_frequency(context: &PlayerContext) -> Result<u64, String> {
        let weights = HashMap::from([
            ("strength", 1.2),
            ("dexterity", 1.6),
            ("vitality", 1.1),
            ("intelligence", 1.3),
            ("spirit", 1.0),
            ("luck", 0.8),
        ]);

        Self::calculate_weighted_frequency(context, 20, &weights, 0.9)
    }

    fn get_cin_amount(player_id: Uuid) -> Result<u64, String> {
        let server = GameServer::global();

        let compass_slot = server.slots_store
            .find_by(|slot| {
                slot.player_id == player_id
                    && slot.kind == crate::models::SlotKind::Compass
                    && slot.item.is_some()
            })
            .ok_or("No compass equipped")?;

        let compass = compass_slot.item.as_ref().unwrap();

        let base_cin = 3u64;

        let level_bonus = compass.level as u64 * 2;

        let enchant_bonus = compass.enchanted as u64 * 1;

        let total_cin = base_cin + level_bonus + enchant_bonus;

        let mut rng = rand::thread_rng();
        let variance_factor = rng.gen_range(0.5..=1.5); // 50% to 150% of original
        let final_cin = (total_cin as f64 * variance_factor).round() as u64;

        Ok(final_cin.max(1))
    }
}

struct PlayerContext {
    player: Player,
    attributes: PlayerAttributes,
    stats: PlayerStats,
    equipment: Vec<Slot>,
    level: u8,
    compass: Option<Item>,
}

pub trait PlayerProbabilities {
    fn exp_chance(&self) -> f32;
    fn loot_chance(&self) -> f32;
    fn exp_frequency(&self) -> u64;
    fn loot_frequency(&self) -> u64;
    fn exp_amount(&self) -> u64;
    fn cin_chance(&self) -> f32;
    fn cin_frequency(&self) -> u64;
    fn cin_amount(&self) -> u64;
}

impl PlayerProbabilities for Uuid {
    fn exp_chance(&self) -> f32 {
        ProbabilityCalculator::exp_gain_chance(*self)
    }

    fn loot_chance(&self) -> f32 {
        ProbabilityCalculator::loot_drop_chance(*self)
    }

    fn exp_frequency(&self) -> u64 {
        ProbabilityCalculator::exp_roll_frequency(*self)
    }

    fn loot_frequency(&self) -> u64 {
        ProbabilityCalculator::loot_roll_frequency(*self)
    }

    fn exp_amount(&self) -> u64 {
        ProbabilityCalculator::calculate_exp_amount(*self)
    }

    fn cin_chance(&self) -> f32 {
        ProbabilityCalculator::cin_gain_chance(*self)
    }

    fn cin_frequency(&self) -> u64 {
        ProbabilityCalculator::cin_roll_frequency(*self)
    }

    fn cin_amount(&self) -> u64 {
        ProbabilityCalculator::calculate_cin_amount(*self)
    }
}