use crate::auth;
use crate::messages::{IncomingMessage, OutgoingEvent, OutgoingMessage};
use crate::meta::Meta;
use crate::server::message_handler::MessageHandler;
use crate::server::websocket_manager::WebSocketManager;
use crate::server::GameServer;
use axum::extract::ws::{Message, WebSocket};
use axum::extract::{Query, State, WebSocketUpgrade};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use futures_util::{SinkExt, StreamExt};
use serde::Deserialize;
use std::sync::Arc;
use tokio::sync::mpsc;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct WsParams {
    token: String,
}

pub async fn websocket_handler(
    ws: WebSocketUpgrade,
    Query(params): Query<WsParams>,
    State(server): State<Arc<GameServer>>,
) -> Result<impl IntoResponse, StatusCode> {
    let claims = auth::verify_token(&params.token)
        .map_err(|_| StatusCode::UNAUTHORIZED)?;

    let user_exists = server.player_store.find_by(|u| u.id == claims.sub).is_some();
    if !user_exists {
        return Err(StatusCode::UNAUTHORIZED);
    }

    Ok(ws.on_upgrade(move |socket| handle_socket(socket, claims.sub, claims.username)))
}

async fn handle_socket(
    socket: WebSocket,
    player_id: Uuid,
    username: String,
) {
    let (ws_sender, mut ws_receiver) = socket.split();
    let (tx, mut rx) = mpsc::unbounded_channel();

    let ws_manager = WebSocketManager::global();
    ws_manager.add_connection(player_id, username.clone(), tx);

    let ws_sender_task = tokio::spawn(async move {
        let mut ws_sender = ws_sender;
        while let Some(msg) = rx.recv().await {
            if ws_sender.send(msg).await.is_err() {
                break;
            }
        }
    });

    send_initial_data_to_user(player_id, &username).await;

    let handler = MessageHandler::new(player_id);

    while let Some(msg) = ws_receiver.next().await {
        match msg {
            Ok(Message::Text(text)) => {
                match serde_json::from_str::<IncomingMessage<serde_json::Value>>(&text) {
                    Ok(msg) => {
                        let data_str = if let Some(data) = msg.data {
                            if let Ok(data_json) = serde_json::to_string(&data) {
                                data_json
                            } else {
                                data.to_string()
                            }
                        } else {
                            "{}".to_string()
                        };

                        match handler.handle(msg.event, &data_str).await {
                            Ok(responses) => {
                                for response in responses {
                                    ws_manager.send_to_player(player_id, response).await;
                                }
                            }
                            Err(e) => {
                                ws_manager.send_log_to_player(player_id, format!("{}", e)).await;
                            }
                        }
                    }
                    Err(e) => {
                        println!("Failed to parse message from {}: {}", username, e);
                    }
                }
            }
            Ok(Message::Close(_)) => break,
            Err(e) => {
                println!("WebSocket error for {}: {}", username, e);
                break;
            }
            _ => {}
        }
    }

    ws_manager.remove_connection(&player_id, &username);
    ws_sender_task.abort();
}

async fn send_initial_data_to_user(player_id: Uuid, username: &str) {
    let server = GameServer::global();
    let ws_manager = WebSocketManager::global();

    ws_manager.send_log_to_player(player_id, format!("Welcome {}!", username)).await;

    if let Some(player) = server.player_store.find_by(|p| p.id == player_id) {
        let msg = OutgoingMessage::new(OutgoingEvent::PlayerInfo, Box::new(player) as Box<dyn erased_serde::Serialize + Send>);
        ws_manager.send_to_player(player_id, msg).await;
    }

    if let Some(player_resource) = server.player_resource_store.find_by(|p| p.player_id == player_id) {
        let msg = OutgoingMessage::new(OutgoingEvent::PlayerResource, Box::new(player_resource) as Box<dyn erased_serde::Serialize + Send>);
        ws_manager.send_to_player(player_id, msg).await;
    }

    if let Some(player_attributes) = server.player_attributes_store.find_by(|p| p.player_id == player_id) {
        let msg = OutgoingMessage::new(OutgoingEvent::PlayerAttributes, Box::new(player_attributes) as Box<dyn erased_serde::Serialize + Send>);
        ws_manager.send_to_player(player_id, msg).await;
    }

    if let Some(player_state) = server.player_state_store.find_by(|p| p.player_id == player_id) {
        let msg = OutgoingMessage::new(OutgoingEvent::PlayerState, Box::new(player_state) as Box<dyn erased_serde::Serialize + Send>);
        ws_manager.send_to_player(player_id, msg).await;
    }

    if let Some(player_stats) = server.player_stats_store.find_by(|p| p.player_id == player_id) {
        let msg = OutgoingMessage::new(OutgoingEvent::PlayerStats, Box::new(player_stats) as Box<dyn erased_serde::Serialize + Send>);
        ws_manager.send_to_player(player_id, msg).await;
    }

    let mut slots = server.slots_store.find_all_by(|p| p.player_id == player_id);
    slots.sort_by_key(|p| p.index);
    let msg = OutgoingMessage::new(OutgoingEvent::Slots, Box::new(slots) as Box<dyn erased_serde::Serialize + Send>);
    ws_manager.send_to_player(player_id, msg).await;

    let msg = OutgoingMessage::new(OutgoingEvent::Meta, Box::new(Meta::new()) as Box<dyn erased_serde::Serialize + Send>);
    ws_manager.send_to_player(player_id, msg).await;
}
