use crate::auth;
use crate::models::{ExpeditionKind, Item, ItemKind, ItemStats, ItemTier, Player, PlayerAttributes, PlayerResource, PlayerState, PlayerStats, Slot, SlotKind};
use crate::server::GameServer;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use strum::IntoEnumIterator;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct RegisterRequest {
    username: String,
    email: String,
    password: String,
}

#[derive(Deserialize)]
pub struct LoginRequest {
    username: String,
    password: String,
}

#[derive(Serialize)]
pub struct AuthResponse {
    token: String,
    player_id: Uuid,
    username: String,
}

pub async fn register(
    State(server): State<Arc<GameServer>>,
    Json(req): Json<RegisterRequest>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let exists = server.player_store.data.iter()
        .any(|entry| entry.name == req.username);

    if exists {
        return Err((StatusCode::CONFLICT, "Username already exists".to_string()));
    }

    let player = Player::new(req.username, req.email, &req.password)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e))?;

    let player_id = player.id;
    let username = player.name.clone();

    server.player_store.insert(player)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e))?;

    let player_resource = PlayerResource::new(player_id);
    server.player_resource_store.insert(player_resource)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e))?;

    let player_attributes = PlayerAttributes::new(player_id, 10, 7, 6, 5, 5, 3);
    server.player_attributes_store.insert(player_attributes)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e))?;

    let player_state = PlayerState::new(player_id);
    server.player_state_store.insert(player_state)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e))?;

    let player_stats = PlayerStats::new(player_id);
    server.player_stats_store.insert(player_stats)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e))?;

    for kind in SlotKind::iter() {
        let qty = match kind {
            SlotKind::Inventory => 56,
            SlotKind::Rune => 24,
            SlotKind::Consumable => 12,
            SlotKind::Ground => 36,
            SlotKind::Ring | SlotKind::Earring => 2,
            _ => 1,
        };

        for index in 0..qty {
            let slot = Slot::new(player_id, kind.clone(), index);
            server.slots_store.insert(slot)
                .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e))?;
        }
    }

    let training_sword_stats = ItemStats::new(Some(20), Some(200), None, None, None, None);
    let training_sword = Item::new(
        player_id,
        ItemKind::Weapon,
        "Training Sword",
        ItemTier::Common,
        "game-icons:broadsword",
        1,
        0,
        0,
        "Basic wooden sword used for beginner combat training. Light and perfect for practice.",
        2f32,
        false,
        false,
        Some(training_sword_stats),
    );
    server.items_store.insert(training_sword.clone())
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e))?;
    training_sword.add_to_empty_slot(SlotKind::Inventory)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e))?;

    let hunter_compass_stats = ItemStats::new(None, None, None, None, Some(60000), Some(ExpeditionKind::Hunt));
    let hunter_compass = Item::new(
        player_id,
        ItemKind::Compass,
        "Hunter Compass",
        ItemTier::Common,
        "game-icons:compass",
        1,
        1,
        0,
        "A compass used for hunting. It can be used to track down a target and find their location.",
        1f32,
        false,
        false,
        Some(hunter_compass_stats),
    );
    server.items_store.insert(hunter_compass.clone())
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e))?;
    hunter_compass.add_to_empty_slot(SlotKind::Inventory)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e))?;

    let token = auth::create_token(player_id, username.clone())
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e))?;

    Ok(Json(AuthResponse {
        token,
        player_id,
        username,
    }))
}

pub async fn login(
    State(server): State<Arc<GameServer>>,
    Json(req): Json<LoginRequest>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let player = server.player_store.data.iter()
        .find(|entry| entry.name == req.username)
        .map(|entry| entry.value().clone())
        .ok_or((StatusCode::UNAUTHORIZED, "Invalid credentials".to_string()))?;

    if !player.verify_password(&req.password) {
        return Err((StatusCode::UNAUTHORIZED, "Invalid credentials".to_string()));
    }

    let token = auth::create_token(player.id, player.name.clone())
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e))?;

    Ok(Json(AuthResponse {
        token,
        player_id: player.id,
        username: player.name,
    }))
}