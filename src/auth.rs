// Arquivo: src/auth.rs

use jsonwebtoken::{encode, decode, Header, Validation, EncodingKey, DecodingKey};
use serde::{Serialize, Deserialize};
use std::env;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
}

pub fn create_jwt(user_id: Uuid) -> String {
    let secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    let claims = Claims {
        sub: user_id.to_string(),
        exp: 2000000000,
    };
    encode(&Header::default(), &claims, &EncodingKey::from_secret(secret.as_bytes())).unwrap()
}

pub fn verify_jwt(token: &str) -> Option<Uuid> {
    let secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    decode::<Claims>(token, &DecodingKey::from_secret(secret.as_bytes()), &Validation::default())
        .ok()
        .and_then(|data| Uuid::parse_str(&data.claims.sub).ok())
}
