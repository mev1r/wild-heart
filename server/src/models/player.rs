use bcrypt::{hash, verify, DEFAULT_COST};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Player {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    #[serde(skip_serializing)]
    pub password_hash: String,
    pub exp: u64,
    pub timestamp: DateTime<Utc>,
}

impl Player {
    pub fn new(name: String, email: String, password: &str) -> Result<Self, String> {
        let password_hash = hash(password, DEFAULT_COST)
            .map_err(|e| format!("Failed to hash password: {}", e))?;

        Ok(Self {
            id: Uuid::new_v4(),
            name,
            email,
            password_hash,
            exp: 0,
            timestamp: Utc::now(),
        })
    }

    pub fn verify_password(&self, password: &str) -> bool {
        verify(password, &self.password_hash).unwrap_or(false)
    }
}

impl super::Model for Player {
    fn id(&self) -> Uuid {
        self.id
    }
}