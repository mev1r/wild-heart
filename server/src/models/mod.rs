mod player;
mod player_resource;
mod slot;
mod item;
mod player_attributes;
mod player_state;
mod player_stats;
mod chat_message;
mod item_stats;
mod expedition;
mod log;

pub use chat_message::ChatMessage;
pub use expedition::Expedition;
pub use item::Item;
pub use item_stats::ItemStats;
pub use log::Log;
pub use player::Player;
pub use player_attributes::PlayerAttributes;
pub use player_resource::PlayerResource;
pub use player_state::PlayerState;
pub use player_stats::PlayerStats;
pub use slot::Slot;

pub use chat_message::ChatKind;
pub use expedition::ExpeditionKind;
pub use item::ItemKind;
pub use item::ItemTier;
pub use slot::SlotKind;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub trait Model: Serialize + for<'de> Deserialize<'de> + Clone + Send + Sync + 'static {
    fn id(&self) -> Uuid;
}