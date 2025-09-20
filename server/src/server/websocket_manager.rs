use crate::messages::{OutgoingEvent, OutgoingMessage};
use crate::models::Log;
use axum::extract::ws::Message;
use dashmap::DashMap;
use std::sync::Arc;
use tokio::sync::mpsc;
use uuid::Uuid;

pub type WebSocketSender = mpsc::UnboundedSender<Message>;

#[derive(Clone)]
#[derive(Debug)]
pub struct WebSocketManager {
    connections: Arc<DashMap<Uuid, WebSocketSender>>,
    player_names: Arc<DashMap<String, Uuid>>,
}

impl WebSocketManager {
    pub fn new() -> Self {
        Self {
            connections: Arc::new(DashMap::new()),
            player_names: Arc::new(DashMap::new()),
        }
    }

    pub fn add_connection(&self, user_id: Uuid, player_name: String, sender: WebSocketSender) {
        self.connections.insert(user_id, sender);
        self.player_names.insert(player_name, user_id);
    }

    pub fn remove_connection(&self, user_id: &Uuid, player_name: &str) {
        self.connections.remove(user_id);
        self.player_names.remove(player_name);
    }

    pub async fn send_to_player(&self, player_id: Uuid, message: OutgoingMessage<Box<dyn erased_serde::Serialize + Send>>) {
        if let Some(sender) = self.connections.get(&player_id) {
            if let Ok(serialized) = serde_json::to_string(&message) {
                let _ = sender.send(Message::Text(serialized));
            }
        }
    }

    pub async fn send_to_player_by_name(&self, player_name: &str, message: OutgoingMessage<Box<dyn erased_serde::Serialize + Send>>) {
        if let Some(player_id) = self.player_names.get(player_name) {
            self.send_to_player(*player_id, message).await;
        }
    }

    pub async fn broadcast_to_all(&self, message: OutgoingMessage<Box<dyn erased_serde::Serialize + Send>>) {
        if let Ok(serialized) = serde_json::to_string(&message) {
            for connection in self.connections.iter() {
                let _ = connection.value().send(Message::Text(serialized.clone()));
            }
        }
    }

    pub fn is_player_online(&self, player_name: &str) -> bool {
        self.player_names.contains_key(player_name)
    }
}

static WEBSOCKET_MANAGER: once_cell::sync::OnceCell<WebSocketManager> = once_cell::sync::OnceCell::new();

impl WebSocketManager {
    pub fn global() -> &'static WebSocketManager {
        WEBSOCKET_MANAGER.get().expect("WebSocketManager not initialized")
    }

    pub fn initialize() -> WebSocketManager {
        let manager = WebSocketManager::new();
        WEBSOCKET_MANAGER.set(manager.clone()).expect("WebSocketManager already initialized");
        manager
    }
}

impl WebSocketManager {
    pub async fn send_log_to_player(&self, player_id: Uuid, text: String) {
        let msg = OutgoingMessage::new(
            OutgoingEvent::Log,
            Box::new(Log::new(text)) as Box<dyn erased_serde::Serialize + Send>,
        );
        self.send_to_player(player_id, msg).await;
    }

    #[allow(dead_code)]
    pub async fn broadcast_log(&self, text: String) {
        let msg = OutgoingMessage::new(
            OutgoingEvent::Log,
            Box::new(Log::new(text)) as Box<dyn erased_serde::Serialize + Send>,
        );
        self.broadcast_to_all(msg).await;
    }
}

