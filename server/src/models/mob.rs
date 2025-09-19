use crate::messages::{OutgoingEvent, OutgoingMessage};
use crate::meta::BASE_MOB_ATTACK;
use crate::server::{GameServer, WebSocketManager};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Mob {
    pub id: Uuid,
    pub expedition_id: Uuid,
    pub name: String,
    pub tier: MobTier,
    pub hp: u64,
    pub max_hp: u64,
    pub level: u64,
    pub damage: u64,
}

impl Mob {
    pub fn new(name: &str, expedition_id: Uuid, hp: u64, tier: MobTier, level: u64) -> Self {
        Self {
            id: Uuid::new_v4(),
            expedition_id,
            name: name.to_string(),
            tier,
            level,
            hp,
            max_hp: hp,
            damage: BASE_MOB_ATTACK,
        }
    }

    pub async fn attack(&self) -> Result<(), String> {
        let server = GameServer::global();
        let ws_manager = WebSocketManager::global();

        let active_expedition = server.expeditions_store
            .find_by(|e| e.id == self.expedition_id && e.ended_at > Utc::now())
            .ok_or("No active expedition to end")?;

        let player_resource = server.player_resource_store
            .find_by(|p| p.player_id == active_expedition.participant_id)
            .ok_or("Player stats not found")?;

        let player_id = player_resource.player_id.clone();

        let updated_resource = server.player_resource_store.update(&player_resource.id, |resource| {
            resource.hp -= self.damage;
        })?;

        ws_manager.send_to_player(player_id, OutgoingMessage::new(
            OutgoingEvent::PlayerResource,
            Box::new(updated_resource) as Box<dyn erased_serde::Serialize + Send>,
        )).await;

        Ok(())
    }
}

impl super::Model for Mob {
    fn id(&self) -> Uuid {
        self.id
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum MobTier {
    Common,
    Magic,
    Rare,
    Epic,
}