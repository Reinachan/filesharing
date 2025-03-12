use crate::models::Permissions;
use serde::{Deserialize, Serialize};
use sqlx::types::chrono::NaiveDateTime;

#[derive(Debug, Clone)]
pub struct UserDB {
    pub username: String,
    pub password: String,
    pub terminate: Option<NaiveDateTime>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct User {
    pub username: String,
    pub password: String,
    pub terminate: Option<NaiveDateTime>,
    pub permissions: Permissions,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct UserWithoutPassword {
    pub username: String,
    pub terminate: Option<NaiveDateTime>,
    pub permissions: Permissions,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct UsernamePassword {
    pub username: String,
    pub password: String,
}
