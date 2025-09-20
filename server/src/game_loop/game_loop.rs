use crate::messages::{OutgoingEvent, OutgoingMessage};
use crate::server::GameServer;
use crate::server::WebSocketManager;
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
}