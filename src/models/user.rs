use crate::models::Permissions;
use sqlx::types::chrono::NaiveDateTime;

#[derive(Debug, Clone)]
pub struct UserDB {
    pub username: String,
    pub password: String,
    pub terminate: Option<NaiveDateTime>,
}

#[derive(Debug, Clone)]
pub struct User {
    pub username: String,
    pub password: String,
    pub terminate: Option<NaiveDateTime>,
    pub permissions: Permissions,
}
