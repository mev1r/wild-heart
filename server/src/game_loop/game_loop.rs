use crate::messages::{OutgoingEvent, OutgoingMessage};
use crate::models::{Mob, MobTier};
use crate::server::GameServer;
use crate::server::WebSocketManager;
use chrono::Utc;
use rand::Rng;
use rand_chacha::rand_core::SeedableRng;
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
        let mut tick_interval = interval(Duration::from_millis(50));
        let mut expedition_countdown_timers: HashMap<Uuid, Instant> = HashMap::new();
        let mut expedition_mob_spawn: HashMap<Uuid, bool> = HashMap::new();
        // let mut player_attack_timers: HashMap<Uuid, Instant> = HashMap::new();

        while self.running {
            tick_interval.tick().await;
            let now = Instant::now();

            self.handle_hp_regeneration(&mut player_regen_timers, now).await;
            self.handle_expeditions(&mut expedition_countdown_timers, now).await;
            self.handle_mobs(&mut expedition_mob_spawn).await;
            // self.handle_player_auto_attack(&mut player_attack_timers, now).await;
        }
    }

    async fn handle_hp_regeneration(&self, player_regen_timers: &mut HashMap<Uuid, Instant>, now: Instant) {
        let server = GameServer::global();
        let ws_manager = WebSocketManager::global();

        let players_with_resources: Vec<_> = server.player_resource_store
            .data
            .iter()
            .map(|entry| entry.value().clone())
            .collect();

        for player_resource in players_with_resources {
            if player_resource.hp >= player_resource.max_hp {
                continue;
            }

            let player_stats = server.player_stats_store
                .find_by(|stats| stats.player_id == player_resource.player_id);

            if let Some(stats) = player_stats {
                let regen_interval = Duration::from_millis(stats.hp_regeneration_interval);

                let should_regen = player_regen_timers
                    .get(&player_resource.player_id)
                    .map(|last_regen| now.duration_since(*last_regen) >= regen_interval)
                    .unwrap_or(true);

                if should_regen {
                    let new_hp = (player_resource.hp + stats.hp_regeneration)
                        .min(player_resource.max_hp);

                    if let Ok(updated_resource) = server.player_resource_store.update(&player_resource.id, |resource| {
                        resource.hp = new_hp;
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

    async fn handle_expeditions(&self, expedition_countdown_timers: &mut HashMap<Uuid, Instant>, now: Instant) {
        let server = GameServer::global();
        let ws_manager = WebSocketManager::global();

        let chrono_now = Utc::now();
        let active_expeditions: Vec<_> = server.expeditions_store
            .data
            .iter()
            .map(|entry| entry.value().clone())
            .filter(|e| {
                let time_since_end = chrono_now - e.ended_at;
                e.ended_at > chrono_now || time_since_end.num_seconds() == 0
            })
            .collect();

        for expedition in active_expeditions {
            let should_send = expedition_countdown_timers
                .get(&expedition.id)
                .map(|last_send| now.duration_since(*last_send) >= Duration::from_secs(1))
                .unwrap_or(true);

            if should_send {
                let remaining = expedition.ended_at - chrono_now;
                let remaining_seconds = remaining.num_seconds().max(0) as u64;

                let msg = OutgoingMessage::new(
                    OutgoingEvent::ExpeditionCountdown,
                    Box::new(remaining_seconds) as Box<dyn erased_serde::Serialize + Send>,
                );

                ws_manager.send_to_player(expedition.participant_id, msg).await;
                expedition_countdown_timers.insert(expedition.id, now);

                if remaining_seconds == 0 {
                    let player_state = server.player_state_store
                        .find_by(|state| state.player_id == expedition.participant_id);

                    if let Some(player_state) = player_state {
                        let _ = server.player_state_store.update(&player_state.id, |state| {
                            state.is_attacking = false;
                            state.is_looting = false;
                        });

                        let player_state = server.player_state_store
                            .find_by(|stats| stats.player_id == expedition.participant_id);

                        if let Some(player_state) = player_state {
                            ws_manager.send_to_player(expedition.participant_id, OutgoingMessage::new(
                                OutgoingEvent::PlayerState,
                                Box::new(player_state) as Box<dyn erased_serde::Serialize + Send>,
                            )).await;
                        }
                    }
                }
            }
        }
    }

    async fn handle_mobs(&self, expedition_mob_spawn: &mut HashMap<Uuid, bool>) {
        let server = GameServer::global();
        let ws_manager = WebSocketManager::global();

        let chrono_now = Utc::now();
        let active_expeditions: Vec<_> = server.expeditions_store
            .data
            .iter()
            .map(|entry| entry.value().clone())
            .filter(|e| e.ended_at > chrono_now)
            .collect();

        for expedition in active_expeditions.clone() {
            if expedition_mob_spawn.get(&expedition.id).copied().unwrap_or(false) {
                continue;
            }

            let existing_mobs = server.mobs_store
                .find_all_by(|mob| mob.expedition_id == expedition.id);

            if !existing_mobs.is_empty() {
                expedition_mob_spawn.insert(expedition.id, true);
                continue;
            }

            let mob_names = vec![
                "Goblin Scout",
                "Wolf",
                "Skeleton Warrior",
                "Giant Spider",
                "Orc Grunt",
                "Dark Elf",
                "Zombie",
                "Bandit",
                "Wild Boar",
            ];

            let mut rng = rand_chacha::ChaCha12Rng::from_entropy();

            for name in mob_names.iter() {
                let tier = if rng.gen_range(0..100) < 60 {
                    MobTier::Common
                } else if rng.gen_range(0..100) < 85 {
                    MobTier::Magic
                } else if rng.gen_range(0..100) < 95 {
                    MobTier::Rare
                } else {
                    MobTier::Epic
                };

                let level = rng.gen_range(1..=5);

                let mob = Mob::new(name, expedition.id, 100, tier, level);
                if let Err(e) = server.mobs_store.insert(mob.clone()) {
                    eprintln!("Failed to spawn mob: {}", e);
                    continue;
                }

                ws_manager.send_to_player(expedition.participant_id, OutgoingMessage::new(
                    OutgoingEvent::Mob,
                    Box::new(mob) as Box<dyn erased_serde::Serialize + Send>,
                )).await;
            }

            expedition_mob_spawn.insert(expedition.id, true);
        }

        let active_expedition_ids: Vec<Uuid> = active_expeditions.iter().map(|e| e.id).collect();
        expedition_mob_spawn.retain(|id, _| active_expedition_ids.contains(id));
    }
}