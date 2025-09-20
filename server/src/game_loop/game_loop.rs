use crate::messages::{OutgoingEvent, OutgoingMessage};
use crate::models::{Item, ItemKind, ItemTier, Slot, SlotKind};
use crate::server::GameServer;
use crate::server::WebSocketManager;
use crate::services::probability_calculator::PlayerProbabilities;
use chrono::Utc;
use std::collections::HashMap;
use std::time::{Duration, Instant};
use tokio::time::interval;
use uuid::Uuid;

pub struct GameLoop {
    running: bool,
}

impl GameLoop {
    pub fn new() -> Self {
        Self { running: false }
    }

    pub async fn start(&mut self) {
        if self.running {
            return;
        }

        self.running = true;

        let mut player_regen_timers: HashMap<Uuid, Instant> = HashMap::new();
        let mut expedition_timers: HashMap<Uuid, Instant> = HashMap::new();
        let mut tick_interval = interval(Duration::from_millis(50));

        while self.running {
            tick_interval.tick().await;
            let now = Instant::now();

            self.handle_expedition(&mut expedition_timers, now).await;
            self.handle_energy_regeneration(&mut player_regen_timers, now).await;
        }
    }

    async fn handle_expedition(&self, expedition_timers: &mut HashMap<Uuid, Instant>, now: Instant) {
        let server = GameServer::global();
        let ws_manager = WebSocketManager::global();

        let active_expeditions = server.expeditions_store.find_all_by(|e| e.ended_at.is_none());
        let now = Instant::now();

        for expedition in active_expeditions {
            let last_tick = expedition_timers
                .get(&expedition.id)
                .copied()
                .unwrap_or_else(|| now - Duration::from_secs(1));

            if now.duration_since(last_tick) >= Duration::from_secs(1) {
                let elapsed_secs = (Utc::now() - expedition.started_at).num_seconds() as u64;

                for player_id in &expedition.participants {
                    ws_manager.send_to_player(*player_id, OutgoingMessage::new(
                        OutgoingEvent::ExpeditionCountup,
                        Box::new(elapsed_secs) as Box<dyn erased_serde::Serialize + Send>,
                    )).await;

                    if let Some(player_resource) = server.player_resource_store.find_by(|r| r.player_id == *player_id) {
                        if player_resource.energy > 0 {
                            let energy_cost = Self::calculate_energy_cost(player_id.clone()).await;

                            let updated_resource = server.player_resource_store.update(&player_resource.id, |resource| {
                                resource.energy = resource.energy.saturating_sub(energy_cost);
                            });

                            if let Ok(updated) = updated_resource {
                                ws_manager.send_to_player(*player_id, OutgoingMessage::new(
                                    OutgoingEvent::PlayerResource,
                                    Box::new(updated.clone()) as Box<dyn erased_serde::Serialize + Send>,
                                )).await;

                                if updated.energy == 0 {
                                    let _ = server.expeditions_store.update(&expedition.id, |exp| {
                                        exp.ended_at = Some(Utc::now());
                                    });

                                    let ground_slots = server.slots_store.find_all_by(|slot| {
                                        slot.player_id == *player_id && slot.kind == SlotKind::Ground
                                    });

                                    for slot in ground_slots {
                                        let _ = server.slots_store.update(&slot.id, |s| {
                                            s.item = None;
                                        });
                                    }

                                    let mut all_slots = server.slots_store.find_all_by(|slot| slot.player_id == *player_id);
                                    all_slots.sort_by_key(|slot| slot.index);

                                    if let Some(player_state) = server.player_state_store.find_by(|state| state.player_id == *player_id) {
                                        let updated_state = server.player_state_store.update(&player_state.id, |state| {
                                            state.is_looting = false;
                                        });

                                        if let Ok(updated) = updated_state {
                                            ws_manager.send_to_player(*player_id, OutgoingMessage::new(
                                                OutgoingEvent::PlayerState,
                                                Box::new(updated) as Box<dyn erased_serde::Serialize + Send>,
                                            )).await;
                                        }
                                    }

                                    ws_manager.send_to_player(*player_id, OutgoingMessage::new(
                                        OutgoingEvent::Slots,
                                        Box::new(all_slots) as Box<dyn erased_serde::Serialize + Send>,
                                    )).await;

                                    ws_manager.send_to_player(*player_id, OutgoingMessage::new(
                                        OutgoingEvent::ExpeditionCountup,
                                        Box::new(-1) as Box<dyn erased_serde::Serialize + Send>,
                                    )).await;

                                    ws_manager.send_log_to_player(*player_id, "Your expedition ended due to lack of energy.".to_string()).await;
                                }
                            }
                        }
                    }

                    Self::handle_exp_rolls(*player_id, elapsed_secs).await;
                    Self::handle_cin_rolls(*player_id, elapsed_secs).await;
                    Self::handle_auto_looting(*player_id).await;
                }

                expedition_timers.insert(expedition.id, now);
            }
        }
    }

    async fn handle_energy_regeneration(&self, player_regen_timers: &mut HashMap<Uuid, Instant>, now: Instant) {
        let server = GameServer::global();
        let ws_manager = WebSocketManager::global();

        let players_with_resources: Vec<_> = server.player_resource_store
            .data
            .iter()
            .map(|entry| entry.value().clone())
            .collect();

        for player_resource in players_with_resources {
            if player_resource.energy >= player_resource.max_energy {
                continue;
            }

            let player_stats = server.player_stats_store
                .find_by(|stats| stats.player_id == player_resource.player_id);

            if let Some(stats) = player_stats {
                let regen_interval = Duration::from_millis(stats.energy_regeneration_interval);

                let should_regen = player_regen_timers
                    .get(&player_resource.player_id)
                    .map(|last_regen| now.duration_since(*last_regen) >= regen_interval)
                    .unwrap_or(true);

                if should_regen {
                    let new_energy = (player_resource.energy + stats.energy_regeneration)
                        .min(player_resource.max_energy);

                    if let Ok(updated_resource) = server.player_resource_store.update(&player_resource.id, |resource| {
                        resource.energy = new_energy;
                    }) {
                        ws_manager.send_to_player(player_resource.player_id, OutgoingMessage::new(
                            OutgoingEvent::PlayerResource,
                            Box::new(updated_resource) as Box<dyn erased_serde::Serialize + Send>,
                        )).await;
                    }

                    player_regen_timers.insert(player_resource.player_id, now);
                }
            }
        }
    }

    async fn handle_exp_rolls(player_id: Uuid, elapsed_secs: u64) {
        let server = GameServer::global();
        let ws_manager = WebSocketManager::global();

        let exp_frequency = player_id.exp_frequency();

        if elapsed_secs % exp_frequency == 0 {
            let exp_chance = player_id.exp_chance();
            let roll = rand::random::<f32>();

            if roll < exp_chance {
                let exp_amount = player_id.exp_amount();

                if let Some(player) = server.player_store.find_by(|p| p.id == player_id) {
                    let updated_player = server.player_store.update(&player.id, |p| {
                        p.exp += exp_amount;
                    });

                    if let Ok(updated) = updated_player {
                        ws_manager.send_to_player(player_id, OutgoingMessage::new(
                            OutgoingEvent::PlayerInfo,
                            Box::new(updated) as Box<dyn erased_serde::Serialize + Send>,
                        )).await;

                        ws_manager.send_to_player(player_id, OutgoingMessage::new(
                            OutgoingEvent::GainedExperience,
                            Box::new(exp_amount) as Box<dyn erased_serde::Serialize + Send>,
                        )).await;

                        ws_manager.send_log_to_player(
                            player_id,
                            format!("You gained {} experience!", exp_amount),
                        ).await;
                    }
                }
            }
        }
    }

    async fn handle_cin_rolls(player_id: Uuid, elapsed_secs: u64) {
        let server = GameServer::global();
        let ws_manager = WebSocketManager::global();

        let cin_frequency = player_id.cin_frequency();

        if elapsed_secs % cin_frequency == 0 {
            let cin_chance = player_id.cin_chance();
            let roll = rand::random::<f32>();

            if roll < cin_chance {
                let cin_amount = player_id.cin_amount();

                let cin_item = Item::new(
                    player_id,
                    ItemKind::Currency,
                    "Cin",
                    ItemTier::Common,
                    "game-icons:two-coins",
                    cin_amount,
                    0,
                    0,
                    "The primary currency of the realm.",
                    0.01,
                    true,
                    false,
                    None,
                );

                match cin_item.add_to_empty_slot(SlotKind::Ground) {
                    Ok(_) => {
                        if let Ok(_) = server.items_store.insert(cin_item) {
                            let mut slots = server.slots_store.find_all_by(|slot| slot.player_id == player_id);
                            slots.sort_by_key(|slot| slot.index);

                            ws_manager.send_to_player(player_id, OutgoingMessage::new(
                                OutgoingEvent::Slots,
                                Box::new(slots) as Box<dyn erased_serde::Serialize + Send>,
                            )).await;

                            ws_manager.send_to_player(player_id, OutgoingMessage::new(
                                OutgoingEvent::GainedCin,
                                Box::new(cin_amount) as Box<dyn erased_serde::Serialize + Send>,
                            )).await;

                            ws_manager.send_log_to_player(
                                player_id,
                                format!("You found {} cin!", cin_amount),
                            ).await;
                        }
                    }
                    Err(_) => {
                        ws_manager.send_log_to_player(
                            player_id,
                            format!("You found {} cin but your backpack is full!", cin_amount),
                        ).await;
                    }
                }
            }
        }
    }

    async fn handle_auto_looting(player_id: Uuid) {
        let server = GameServer::global();
        let ws_manager = WebSocketManager::global();

        let player_state = server.player_state_store
            .find_by(|state| state.player_id == player_id);

        let Some(state) = player_state else {
            return;
        };

        if !state.is_looting {
            return;
        }

        let ground_slots_with_items: Vec<Slot> = server.slots_store
            .find_all_by(|slot| {
                slot.player_id == player_id
                    && slot.kind == SlotKind::Ground
                    && slot.item.is_some()
            });

        let mut slots_updated = false;

        for ground_slot in ground_slots_with_items {
            if let Some(item) = &ground_slot.item {
                match item.add_to_empty_slot(SlotKind::Inventory) {
                    Ok(_) => {
                        let _ = server.items_store.insert(item.clone());

                        let _ = server.slots_store.update(&ground_slot.id, |slot| {
                            slot.item = None;
                        });

                        slots_updated = true;

                        ws_manager.send_log_to_player(
                            player_id,
                            format!("Looted {} {}", item.quantity, item.name),
                        ).await;
                    }
                    Err(_) => {
                        ws_manager.send_log_to_player(
                            player_id,
                            "Inventory is full.".to_string(),
                        ).await;
                    }
                }
            }
        }

        if slots_updated {
            let mut all_slots = server.slots_store.find_all_by(|slot| slot.player_id == player_id);
            all_slots.sort_by_key(|slot| slot.index);

            ws_manager.send_to_player(player_id, OutgoingMessage::new(
                OutgoingEvent::Slots,
                Box::new(all_slots) as Box<dyn erased_serde::Serialize + Send>,
            )).await;
        }
    }

    async fn calculate_energy_cost(player_id: Uuid) -> u64 {
        let server = GameServer::global();

        let compass_slot = server.slots_store
            .find_by(|slot| {
                slot.player_id == player_id
                    && slot.kind == SlotKind::Compass
                    && slot.item.is_some()
            });

        if let Some(slot) = compass_slot {
            if let Some(compass) = &slot.item {
                let base_cost = 4u64;

                let level_cost = (compass.level as f64 * 0.5).round() as u64;

                let enchant_cost = (compass.enchanted as f64 * 0.3).round() as u64;

                let total_cost = base_cost + level_cost + enchant_cost;

                return total_cost.max(4);
            }
        }

        4
    }
}