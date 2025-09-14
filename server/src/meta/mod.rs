mod level_to_exp;
mod base_stats;
mod mob;

pub use base_stats::*;
pub use level_to_exp::level_to_exp;
pub use mob::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Meta {
    pub level_to_exp: HashMap<u8, u64>,
    pub base_stats: BaseStats,
}

impl Meta {
    pub fn new() -> Self {
        Self {
            level_to_exp: level_to_exp(),
            base_stats: BaseStats::new(),
        }
    }
}