use crate::game_loop::GameLoop;
use crate::models::{ChatMessage, Expedition, Item, Player, PlayerAttributes, PlayerResource, PlayerState, PlayerStats, Slot};
use crate::server::GameServer;
use crate::store::Store;
use std::sync::Arc;

mod models;
mod store;
mod server;
mod auth;
mod messages;
mod meta;
mod game_loop;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();

    let db = sled::open("./game_data").map_err(|e| format!("sled open failed: {e}"))?;

    let player_store: Store<Player> = Store::with_persistence(
        db.clone(),
        "players",
    )?;

    let player_resource_store: Store<PlayerResource> = Store::with_persistence(
        db.clone(),
        "player_resources",
    )?;

    let player_attributes_store: Store<PlayerAttributes> = Store::with_persistence(
        db.clone(),
        "player_attributes",
    )?;

    let player_state_store: Store<PlayerState> = Store::with_persistence(
        db.clone(),
        "player_states",
    )?;

    let player_stats_store: Store<PlayerStats> = Store::with_persistence(
        db.clone(),
        "player_stats",
    )?;

    let items_store: Store<Item> = Store::with_persistence(
        db.clone(),
        "items",
    )?;

    let slots_store: Store<Slot> = Store::with_persistence(
        db.clone(),
        "slots",
    )?;

    let chat_store: Store<ChatMessage> = Store::with_persistence(
        db.clone(),
        "chat_messages",
    )?;

    let expeditions_store: Store<Expedition> = Store::with_persistence(
        db.clone(),
        "expeditions",
    )?;

    let game_server = Arc::new(GameServer::new(
        player_store,
        player_resource_store,
        player_attributes_store,
        player_state_store,
        player_stats_store,
        items_store,
        slots_store,
        chat_store,
        expeditions_store,
    ));

    GameServer::initialize_global(game_server.clone())
        .expect("Failed to initialize global GameServer");

    let mut game_loop = GameLoop::new();
    tokio::spawn(async move {
        game_loop.start().await;
    });

    let app = game_server.create_router();
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.map_err(|e| format!("bind failed: {e}"))?;

    axum::serve(listener, app).await?;

    Ok(())
}
