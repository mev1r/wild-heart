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
        let mut player_attack_timers: HashMap<Uuid, Instant> = HashMap::new();
        let mut mob_attack_timers: HashMap<Uuid, Instant> = HashMap::new();
        let mut mob_respawn_timers: HashMap<Uuid, (Instant, Mob)> = HashMap::new();
        let mut engaged_mobs: HashMap<Uuid, bool> = HashMap::new();

        while self.running {
            tick_interval.tick().await;
            let now = Instant::now();

            self.handle_hp_regeneration(&mut player_regen_timers, now).await;
            self.handle_expeditions(&mut expedition_countdown_timers, now).await;
            self.handle_mobs(&mut expedition_mob_spawn).await;
            self.handle_player_auto_attack(&mut player_attack_timers, &mut engaged_mobs, now).await;
            self.handle_mob_attacks(&mut mob_attack_timers, &engaged_mobs, now).await;
            self.handle_mob_respawning(&mut mob_respawn_timers, now).await;
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
                            state.target_id = None;
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

                    let expedition_mobs: Vec<_> = server.mobs_store
                        .find_all_by(|mob| mob.expedition_id == expedition.id);

                    for mob in expedition_mobs {
                        let _ = server.mobs_store.update(&mob.id, |m| {
                            m.hp = 0;
                            m.max_hp = 0;
                        });
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
                .find_all_by(|mob| mob.expedition_id == expedition.id && mob.max_hp > 0);

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

                let mob = Mob::new(name, expedition.id, 1000, tier, level);
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

    async fn handle_player_auto_attack(&self, player_attack_timers: &mut HashMap<Uuid, Instant>, engaged_mobs: &mut HashMap<Uuid, bool>, now: Instant) {
        let server = GameServer::global();
        let ws_manager = WebSocketManager::global();

        let attacking_players: Vec<_> = server.player_state_store
            .data
            .iter()
            .filter(|entry| entry.value().is_attacking)
            .map(|entry| entry.value().clone())
            .collect();

        for player_state in attacking_players {
            let active_expedition = server.expeditions_store
                .find_by(|e| e.participant_id == player_state.player_id && e.ended_at > Utc::now());

            if active_expedition.is_none() {
                continue;
            }

            let expedition = active_expedition.unwrap();

            let player_stats = server.player_stats_store
                .find_by(|stats| stats.player_id == player_state.player_id);

            if let Some(stats) = player_stats {
                let attack_interval = Duration::from_millis(stats.attack_speed);

                let should_attack = player_attack_timers
                    .get(&player_state.player_id)
                    .map(|last_attack| now.duration_since(*last_attack) >= attack_interval)
                    .unwrap_or(true);

                if should_attack {
                    let mut target_mob = server.mobs_store
                        .find_by(|mob| mob.expedition_id == expedition.id && mob.hp > 0 && mob.max_hp > 0);

                    if let Some(current_target_id) = &player_state.target_id {
                        if let Some(current_target) = server.mobs_store.find_by(|mob| mob.id == *current_target_id && mob.hp > 0) {
                            target_mob = Some(current_target);
                        }
                    }

                    if let Some(mob) = target_mob {
                        let target_changed = player_state.target_id != Some(mob.id);

                        if target_changed {
                            let updated_state = server.player_state_store.update(&player_state.id, |state| {
                                state.target_id = Some(mob.id);
                            }).unwrap();

                            ws_manager.send_to_player(player_state.player_id, OutgoingMessage::new(
                                OutgoingEvent::PlayerState,
                                Box::new(updated_state) as Box<dyn erased_serde::Serialize + Send>,
                            )).await;
                        }

                        engaged_mobs.insert(mob.id, true);

                        let damage = stats.attack;
                        let new_hp = mob.hp.saturating_sub(damage);

                        if let Ok(updated_mob) = server.mobs_store.update(&mob.id, |m| {
                            m.hp = new_hp;
                        }) {
                            ws_manager.send_to_player(player_state.player_id, OutgoingMessage::new(
                                OutgoingEvent::Mob,
                                Box::new(updated_mob.clone()) as Box<dyn erased_serde::Serialize + Send>,
                            )).await;

                            if new_hp == 0 {
                                let exp_gain = 50 * mob.level;

                                if let Some(player) = server.player_store.find_by(|p| p.id == player_state.player_id) {
                                    let _ = server.player_store.update(&player.id, |p| {
                                        p.exp += exp_gain;
                                    });

                                    if let Some(updated_player) = server.player_store.find_by(|p| p.id == player_state.player_id) {
                                        ws_manager.send_to_player(player_state.player_id, OutgoingMessage::new(
                                            OutgoingEvent::PlayerInfo,
                                            Box::new(updated_player) as Box<dyn erased_serde::Serialize + Send>,
                                        )).await;
                                    }
                                }

                                ws_manager.send_log_to_player(
                                    player_state.player_id,
                                    format!("You killed {} and gained {} experience!", mob.name, exp_gain),
                                ).await;

                                engaged_mobs.remove(&mob.id);

                                let updated_state = server.player_state_store.update(&player_state.id, |state| {
                                    state.target_id = None;
                                }).unwrap();

                                ws_manager.send_to_player(player_state.player_id, OutgoingMessage::new(
                                    OutgoingEvent::PlayerState,
                                    Box::new(updated_state) as Box<dyn erased_serde::Serialize + Send>,
                                )).await;

                                let _ = Mob::new(&mob.name, mob.expedition_id, mob.max_hp, mob.tier.clone(), mob.level);
                            } else {
                                ws_manager.send_log_to_player(
                                    player_state.player_id,
                                    format!("You attack {} for {} damage!", mob.name, damage),
                                ).await;
                            }
                        }
                    }

                    player_attack_timers.insert(player_state.player_id, now);
                }
            }
        }
    }

    async fn handle_mob_attacks(&self, mob_attack_timers: &mut HashMap<Uuid, Instant>, engaged_mobs: &HashMap<Uuid, bool>, now: Instant) {
        let server = GameServer::global();
        let ws_manager = WebSocketManager::global();

        let chrono_now = Utc::now();
        let active_expeditions: Vec<_> = server.expeditions_store
            .data
            .iter()
            .map(|entry| entry.value().clone())
            .filter(|e| e.ended_at > chrono_now)
            .collect();

        for expedition in active_expeditions {
            let living_mobs: Vec<_> = server.mobs_store
                .find_all_by(|mob| {
                    mob.expedition_id == expedition.id
                        && mob.hp > 0
                        && engaged_mobs.get(&mob.id).copied().unwrap_or(false)
                });

            let player_resource = server.player_resource_store
                .find_by(|resource| resource.player_id == expedition.participant_id);

            let player_is_alive = player_resource
                .map(|resource| resource.hp > 0)
                .unwrap_or(false);

            if !player_is_alive {
                continue;
            }

            for mob in living_mobs {
                let mut rng = rand_chacha::ChaCha12Rng::from_entropy();
                let attack_interval = Duration::from_millis(rng.gen_range(3000..=5000));

                let should_attack = mob_attack_timers
                    .get(&mob.id)
                    .map(|last_attack| now.duration_since(*last_attack) >= attack_interval)
                    .unwrap_or(true);

                if should_attack {
                    if let Err(e) = mob.attack().await {
                        eprintln!("Mob attack failed: {}", e);
                    } else {
                        ws_manager.send_log_to_player(
                            expedition.participant_id,
                            format!("{} attacks you for {} damage!", mob.name, mob.damage),
                        ).await;
                    }

                    mob_attack_timers.insert(mob.id, now);
                }
            }
        }

        let all_living_engaged_mob_ids: Vec<Uuid> = server.mobs_store
            .data
            .iter()
            .filter(|entry| {
                let mob = entry.value();
                mob.hp > 0 && engaged_mobs.get(&mob.id).copied().unwrap_or(false)
            })
            .map(|entry| entry.value().id)
            .collect();

        mob_attack_timers.retain(|mob_id, _| all_living_engaged_mob_ids.contains(mob_id));
    }

    async fn handle_mob_respawning(&self, mob_respawn_timers: &mut HashMap<Uuid, (Instant, Mob)>, now: Instant) {
        let server = GameServer::global();
        let ws_manager = WebSocketManager::global();

        let dead_mobs: Vec<_> = server.mobs_store
            .data
            .iter()
            .filter(|entry| {
                let mob = entry.value();
                mob.hp == 0 && !mob_respawn_timers.contains_key(&mob.id)
            })
            .map(|entry| entry.value().clone())
            .collect();

        for dead_mob in dead_mobs {
            let expedition_active = server.expeditions_store
                .find_by(|e| e.id == dead_mob.expedition_id && e.ended_at > Utc::now())
                .is_some();

            if expedition_active {
                let fresh_mob = Mob::new(
                    &dead_mob.name,
                    dead_mob.expedition_id,
                    dead_mob.max_hp,
                    dead_mob.tier.clone(),
                    dead_mob.level,
                );

                mob_respawn_timers.insert(dead_mob.id, (now, fresh_mob));
            }
        }

        let respawn_delay = Duration::from_secs(8);
        let ready_to_respawn: Vec<Uuid> = mob_respawn_timers
            .iter()
            .filter(|(_, (death_time, _))| now.duration_since(*death_time) >= respawn_delay)
            .map(|(mob_id, _)| *mob_id)
            .collect();

        for mob_id in ready_to_respawn {
            if let Some((_, fresh_mob)) = mob_respawn_timers.remove(&mob_id) {
                let expedition_active = server.expeditions_store
                    .find_by(|e| e.id == fresh_mob.expedition_id && e.ended_at > Utc::now())
                    .is_some();

                if expedition_active {
                    if let Ok(respawned_mob) = server.mobs_store.update(&mob_id, |mob| {
                        mob.hp = fresh_mob.hp;
                        mob.max_hp = fresh_mob.max_hp;
                    }) {
                        if let Some(expedition) = server.expeditions_store.find_by(|e| e.id == fresh_mob.expedition_id) {
                            ws_manager.send_to_player(expedition.participant_id, OutgoingMessage::new(
                                OutgoingEvent::Mob,
                                Box::new(respawned_mob.clone()) as Box<dyn erased_serde::Serialize + Send>,
                            )).await;
                        }
                    }
                }
            }
        }

        let active_expedition_ids: Vec<Uuid> = server.expeditions_store
            .data
            .iter()
            .filter(|entry| entry.value().ended_at > Utc::now())
            .map(|entry| entry.value().id)
            .collect();

        mob_respawn_timers.retain(|_, (_, mob)| {
            active_expedition_ids.contains(&mob.expedition_id)
        });
    }
}