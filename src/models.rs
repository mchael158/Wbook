// Arquivo: src/models/user.rs

use serde::{Serialize, Deserialize};
use uuid::Uuid;
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub password_hash: String,
}

#[derive(Debug, Deserialize)]
pub struct RegisterData {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct LoginData {
    pub email: String,
    pub password: String,
}
