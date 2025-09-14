use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

static JWT_SECRET: Lazy<String> = Lazy::new(|| {
    std::env::var("JWT_SECRET")
        .expect("JWT_SECRET must be set in environment variables")
});

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: Uuid,
    pub username: String,
    pub exp: usize,
}

impl Claims {
    pub fn new(user_id: Uuid, username: String) -> Self {
        Self {
            sub: user_id,
            username,
            exp: (chrono::Utc::now() + chrono::Duration::hours(24)).timestamp() as usize,
        }
    }
}

pub fn create_token(user_id: Uuid, username: String) -> Result<String, String> {
    let claims = Claims::new(user_id, username);

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(JWT_SECRET.as_bytes())
    ).map_err(|e| e.to_string())
}

pub fn verify_token(token: &str) -> Result<Claims, String> {
    decode::<Claims>(
        token,
        &DecodingKey::from_secret(JWT_SECRET.as_bytes()),
        &Validation::default()
    )
        .map(|data| data.claims)
        .map_err(|e| e.to_string())
}