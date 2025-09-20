mod websocket;
mod auth_routes;
mod message_handler;
mod websocket_manager;

use crate::models::{ChatMessage, Expedition, Item, Player, PlayerAttributes, PlayerResource, PlayerState, PlayerStats, Slot};
use crate::store::Store;
use axum::http::{header, Method};
use axum::response::IntoResponse;
use axum::routing::{get, post};
use axum::Router;
use once_cell::sync::OnceCell;
use std::sync::Arc;
use tower_http::cors::{Any, CorsLayer};
pub(crate) use websocket_manager::WebSocketManager;

pub struct GameServer {
    pub player_store: Arc<Store<Player>>,
    pub player_resource_store: Arc<Store<PlayerResource>>,
    pub player_attributes_store: Arc<Store<PlayerAttributes>>,
    pub player_state_store: Arc<Store<PlayerState>>,
    pub player_stats_store: Arc<Store<PlayerStats>>,
    pub items_store: Arc<Store<Item>>,
    pub slots_store: Arc<Store<Slot>>,
    pub chat_store: Arc<Store<ChatMessage>>,
    pub expeditions_store: Arc<Store<Expedition>>,
}

static GAME_SERVER: OnceCell<Arc<GameServer>> = OnceCell::new();

impl GameServer {
    pub fn new(
        player_store: Store<Player>,
        player_resource_store: Store<PlayerResource>,
        player_attributes_store: Store<PlayerAttributes>,
        player_state_store: Store<PlayerState>,
        player_stats_store: Store<PlayerStats>,
        items_store: Store<Item>,
        slots_store: Store<Slot>,
        chat_store: Store<ChatMessage>,
        expeditions_store: Store<Expedition>,
    ) -> Self {
        WebSocketManager::initialize();

        Self {
            player_store: Arc::new(player_store),
            player_resource_store: Arc::new(player_resource_store),
            player_attributes_store: Arc::new(player_attributes_store),
            player_state_store: Arc::new(player_state_store),
            player_stats_store: Arc::new(player_stats_store),
            items_store: Arc::new(items_store),
            slots_store: Arc::new(slots_store),
            chat_store: Arc::new(chat_store),
            expeditions_store: Arc::new(expeditions_store),
        }
    }

    pub fn initialize_global(server: Arc<GameServer>) -> Result<(), &'static str> {
        GAME_SERVER.set(server).map_err(|_| "GameServer already initialized")
    }

    pub fn global() -> &'static Arc<GameServer> {
        GAME_SERVER.get().expect("GameServer not initialized")
    }

    pub fn create_router(self: Arc<Self>) -> Router {
        let cors = CorsLayer::new()
            .allow_origin(Any)
            .allow_methods([
                Method::GET,
                Method::POST,
                Method::PUT,
                Method::DELETE,
                Method::OPTIONS,
            ])
            .allow_headers([
                header::CONTENT_TYPE,
                header::AUTHORIZATION,
            ])
            .allow_credentials(false);

        Router::new()
            .route("/health", get(health_check))
            .route("/auth/register", post(auth_routes::register))
            .route("/auth/login", post(auth_routes::login))
            .route("/ws", get(websocket::websocket_handler))
            .layer(cors)
            .with_state(self)
    }
}

async fn health_check() -> impl IntoResponse {
    "OK"
}