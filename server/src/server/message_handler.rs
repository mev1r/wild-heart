use crate::messages::{DropItem, IncomingEvent, OutgoingEvent, OutgoingMessage, SendChatMessage, TakeItem};
use crate::models::{ChatKind, ChatMessage, Expedition, ExpeditionKind, ItemKind, PlayerState, SlotKind};
use crate::server::websocket_manager::WebSocketManager;
use crate::server::GameServer;
use chrono::Utc;
use uuid::Uuid;

pub struct MessageHandler {
    player_id: Uuid,
}

impl MessageHandler {
    pub fn new(
        player_id: Uuid,
    ) -> Self {
        Self {
            player_id,
        }
    }

    pub async fn handle(&self, event: IncomingEvent, data: &str) -> Result<Vec<OutgoingMessage<Box<dyn erased_serde::Serialize + Send>>>, String> {
        match event {
            IncomingEvent::TakeItem => {
                let take_item: TakeItem = serde_json::from_str(data)
                    .map_err(|e| format!("Failed to parse TakeItem data: {}", e))?;
                self.handle_take_item(take_item).await
            }
            IncomingEvent::DropItem => {
                let drop_item: DropItem = serde_json::from_str(data)
                    .map_err(|e| format!("Failed to parse DropItem data: {}", e))?;
                self.handle_drop_item(drop_item).await
            }
            IncomingEvent::SendChatMessage => {
                let chat_message: SendChatMessage = serde_json::from_str(data)
                    .map_err(|e| format!("Failed to parse SendChatMessage data: {}", e))?;
                self.handle_send_chat_message(chat_message).await
            }
            IncomingEvent::StartExpedition => {
                self.handle_start_expedition().await
            }
            IncomingEvent::EndExpedition => {
                self.handle_end_expedition().await
            }
            IncomingEvent::ToggleAttack => {
                self.handle_toggle_attack().await
            }
            IncomingEvent::ToggleLoot => {
                self.handle_toggle_loot().await
            }
        }
    }

    async fn handle_take_item(&self, data: TakeItem) -> Result<Vec<OutgoingMessage<Box<dyn erased_serde::Serialize + Send>>>, String> {
        let server = GameServer::global();
        let slot = server.slots_store
            .find_by(|slot| {
                slot.player_id == self.player_id
                    && slot.kind == data.kind
                    && slot.index == data.index
            })
            .ok_or("Slot not found")?;

        let item = slot.item.as_ref().ok_or("No item in slot")?;

        if data.kind == SlotKind::Compass {
            let has_active_expedition = server.expeditions_store
                .find_by(|e| e.participant_id == self.player_id && e.ended_at > Utc::now())
                .is_some();

            if has_active_expedition {
                return Err("Cannot remove compass during active expedition".to_string());
            }
        }

        let hand_slot = server.slots_store
            .find_by(|slot| {
                slot.player_id == self.player_id
                    && slot.kind == SlotKind::Hand
                    && slot.index == 0
            })
            .ok_or("Hand slot not found")?;

        if hand_slot.item.is_some() {
            return Err("Hand is not empty".to_string());
        }

        let item_to_move = item.clone();

        server.slots_store.update(&slot.id, |slot| {
            slot.item = None;
        })?;

        server.slots_store.update(&hand_slot.id, |slot| {
            slot.item = Some(item_to_move);
        })?;

        if slot.is_equipment_slot() {
            if let Some(current_stats) = server.player_stats_store.find_by(|s| s.player_id == self.player_id) {
                let _ = current_stats.recalculate();
            }
        }

        let mut slots = server.slots_store.find_all_by(|slot| slot.player_id == self.player_id);
        slots.sort_by_key(|slot| slot.index);

        Ok(vec![OutgoingMessage::new(
            OutgoingEvent::Slots,
            Box::new(slots) as Box<dyn erased_serde::Serialize + Send>,
        )])
    }

    async fn handle_drop_item(&self, data: DropItem) -> Result<Vec<OutgoingMessage<Box<dyn erased_serde::Serialize + Send>>>, String> {
        let server = GameServer::global();

        let hand_slot = server.slots_store
            .find_by(|slot| {
                slot.player_id == self.player_id
                    && slot.kind == SlotKind::Hand
                    && slot.index == 0
            })
            .ok_or("Hand slot not found")?;

        let hand_item = hand_slot.item.as_ref().ok_or("No item in hand")?;

        let target_slot = server.slots_store
            .find_by(|slot| {
                slot.player_id == self.player_id
                    && slot.kind == data.kind
                    && slot.index == data.index
            })
            .ok_or("Target slot not found")?;

        if target_slot.kind == SlotKind::Ground {
            return Err("Cannot drop items on the ground".to_string());
        }

        if data.kind == SlotKind::Inventory {
            if let Some(existing_item) = &target_slot.item {
                if hand_item.is_stackable
                    && existing_item.is_stackable
                    && hand_item.kind == existing_item.kind
                    && hand_item.name == existing_item.name
                {
                    let combined_quantity = hand_item.quantity + existing_item.quantity;

                    server.slots_store.update(&target_slot.id, |slot| {
                        if let Some(ref mut item) = slot.item {
                            item.quantity = combined_quantity;
                        }
                    })?;
                } else {
                    let hand_item_clone = hand_item.clone();
                    let existing_item_clone = existing_item.clone();

                    server.slots_store.update(&target_slot.id, |slot| {
                        slot.item = Some(hand_item_clone);
                    })?;

                    server.slots_store.update(&hand_slot.id, |slot| {
                        slot.item = Some(existing_item_clone);
                    })?;
                }
            } else {
                server.slots_store.update(&target_slot.id, |slot| {
                    slot.item = Some(hand_item.clone());
                })?;
            }
            server.slots_store.update(&hand_slot.id, |slot| {
                slot.item = None;
            })?;
        } else {
            let item_matches_slot = match (&hand_item.kind, &data.kind) {
                (ItemKind::Weapon, SlotKind::Weapon) => true,
                (ItemKind::Compass, SlotKind::Compass) => true,
                _ => false,
            };

            if !item_matches_slot {
                return Err("Item type doesn't match slot type".to_string());
            }

            if let Some(existing_item) = &target_slot.item {
                let hand_item_clone = hand_item.clone();
                let existing_item_clone = existing_item.clone();

                server.slots_store.update(&target_slot.id, |slot| {
                    slot.item = Some(hand_item_clone);
                })?;

                server.slots_store.update(&hand_slot.id, |slot| {
                    slot.item = Some(existing_item_clone);
                })?;
            } else {
                server.slots_store.update(&target_slot.id, |slot| {
                    slot.item = Some(hand_item.clone());
                })?;

                server.slots_store.update(&hand_slot.id, |slot| {
                    slot.item = None;
                })?;
            }
        }

        if target_slot.is_equipment_slot() {
            if let Some(current_stats) = server.player_stats_store.find_by(|s| s.player_id == self.player_id) {
                let _ = current_stats.recalculate();
            }
        }

        let mut slots = server.slots_store.find_all_by(|slot| slot.player_id == self.player_id);
        slots.sort_by_key(|slot| slot.index);

        Ok(vec![OutgoingMessage::new(
            OutgoingEvent::Slots,
            Box::new(slots) as Box<dyn erased_serde::Serialize + Send>,
        )])
    }

    async fn handle_send_chat_message(&self, data: SendChatMessage) -> Result<Vec<OutgoingMessage<Box<dyn erased_serde::Serialize + Send>>>, String> {
        let server = GameServer::global();
        let ws_manager = WebSocketManager::global();

        let player = server.player_store
            .find_by(|p| p.id == self.player_id)
            .ok_or("Player not found")?;

        let data_kind = data.kind.clone();
        let recipient_name = data.recipient.clone();

        if data_kind == ChatKind::Whisper {
            if let Some(ref recipient_name) = data.recipient {
                if !ws_manager.is_player_online(recipient_name) {
                    return Err(format!("Player '{}' is not online", recipient_name));
                }
            } else {
                return Err("Whisper messages must have a recipient".to_string());
            }
        }

        let chat_message = ChatMessage::new(
            player.name.clone(),
            data.recipient,
            data.kind,
            data.content,
        );

        server.chat_store.insert(chat_message.clone())
            .map_err(|e| format!("Failed to store chat message: {}", e))?;

        let msg = OutgoingMessage::new(OutgoingEvent::ChatMessage, Box::new(chat_message) as Box<dyn erased_serde::Serialize + Send>);

        match data_kind {
            ChatKind::General | ChatKind::Trade => {
                ws_manager.broadcast_to_all(msg).await;
            }
            ChatKind::Whisper => {
                if let Some(ref recipient) = recipient_name {
                    ws_manager.send_to_player_by_name(recipient, msg).await;
                }
            }
        }

        Ok(vec![])
    }

    async fn handle_start_expedition(&self) -> Result<Vec<OutgoingMessage<Box<dyn erased_serde::Serialize + Send>>>, String> {
        let server = GameServer::global();
        let ws_manager = WebSocketManager::global();

        let already_active = server.expeditions_store
            .find_by(|e| e.participant_id == self.player_id && e.ended_at > Utc::now())
            .is_some();

        if already_active {
            return Err("Expedition already in progress".to_string());
        }

        let mut compass_slot = server.slots_store
            .find_by(|slot| slot.player_id == self.player_id && slot.kind == SlotKind::Compass);

        let Some(slot) = &mut compass_slot else {
            return Err("Compass slot not found".to_string());
        };

        let item = slot.item.clone().ok_or("Compass slot is empty")?;
        if item.kind != ItemKind::Compass {
            return Err("Item in Compass slot is not a compass".to_string());
        }

        let stats = item.stats.as_ref().ok_or("Compass has no stats")?;
        let duration_ms = stats.expedition_duration.ok_or("Compass has no expedition_duration")?;
        let kind = stats.expedition_kind.clone().unwrap_or(ExpeditionKind::Hunt);

        let ended_at = Utc::now() + chrono::Duration::milliseconds(duration_ms as i64);
        let expedition = Expedition::new(self.player_id, kind, ended_at);

        server.expeditions_store.insert(expedition.clone())
            .map_err(|e| format!("Failed to store expedition: {}", e))?;

        ws_manager.send_log_to_player(self.player_id, "You started the expedition".to_string()).await;

        Ok(vec![])
    }

    async fn handle_end_expedition(&self) -> Result<Vec<OutgoingMessage<Box<dyn erased_serde::Serialize + Send>>>, String> {
        let server = GameServer::global();
        let ws_manager = WebSocketManager::global();

        let active = server.expeditions_store
            .find_by(|e| e.participant_id == self.player_id && e.ended_at > Utc::now())
            .ok_or("No active expedition to end")?;

        let _ = server.expeditions_store.update(&active.id, |exp| {
            exp.ended_at = Utc::now();
        })?;

        let player_state = server.player_state_store
            .find_by(|state| state.player_id == self.player_id)
            .ok_or("Player state not found")?;

        let updated_state = server.player_state_store.update(&player_state.id, |state| {
            state.is_attacking = false;
            state.is_looting = false;
        })?;

        ws_manager.send_log_to_player(self.player_id, "You left the expedition".to_string()).await;

        Ok(vec![OutgoingMessage::new(
            OutgoingEvent::PlayerState,
            Box::new(updated_state) as Box<dyn erased_serde::Serialize + Send>,
        )])
    }

    async fn handle_toggle_attack(&self) -> Result<Vec<OutgoingMessage<Box<dyn erased_serde::Serialize + Send>>>, String> {
        let server = GameServer::global();
        let ws_manager = WebSocketManager::global();

        server.expeditions_store
            .find_by(|e| e.participant_id == self.player_id && e.ended_at > Utc::now())
            .ok_or("No active expedition to attack in")?;

        let player_state = server.player_state_store
            .find_by(|state| state.player_id == self.player_id)
            .ok_or("Player state not found")?;

        let updated_state: PlayerState;
        if player_state.is_attacking {
            updated_state = server.player_state_store.update(&player_state.id, |state| { state.is_attacking = false; })?;
            ws_manager.send_log_to_player(self.player_id, "You stopped attacking!".to_string()).await;
        } else {
            updated_state = server.player_state_store.update(&player_state.id, |state| { state.is_attacking = true; })?;
            ws_manager.send_log_to_player(self.player_id, "You started attacking".to_string()).await;
        }

        Ok(vec![OutgoingMessage::new(
            OutgoingEvent::PlayerState,
            Box::new(updated_state) as Box<dyn erased_serde::Serialize + Send>,
        )])
    }

    async fn handle_toggle_loot(&self) -> Result<Vec<OutgoingMessage<Box<dyn erased_serde::Serialize + Send>>>, String> {
        let server = GameServer::global();
        let ws_manager = WebSocketManager::global();

        server.expeditions_store
            .find_by(|e| e.participant_id == self.player_id && e.ended_at > Utc::now())
            .ok_or("No active expedition to loot in")?;

        let player_state = server.player_state_store
            .find_by(|state| state.player_id == self.player_id)
            .ok_or("Player state not found")?;

        let updated_state: PlayerState;
        if player_state.is_looting {
            updated_state = server.player_state_store.update(&player_state.id, |state| { state.is_looting = false; })?;
            ws_manager.send_log_to_player(self.player_id, "You stopped looting items".to_string()).await;
        } else {
            updated_state = server.player_state_store.update(&player_state.id, |state| { state.is_looting = true; })?;
            ws_manager.send_log_to_player(self.player_id, "You started looting items".to_string()).await;
        }

        Ok(vec![OutgoingMessage::new(
            OutgoingEvent::PlayerState,
            Box::new(updated_state) as Box<dyn erased_serde::Serialize + Send>,
        )])
    }
}