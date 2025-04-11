use crate::models::Permissions;
use serde::{Deserialize, Serialize};
use sqlx::types::chrono::NaiveDateTime;

#[derive(Debug, Clone)]
pub struct UserDB {
    pub id: i64,
    pub username: String,
    pub password: String,
    pub terminate: Option<NaiveDateTime>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CreateUserDB {
    pub username: String,
    pub password: String,
    pub terminate: Option<NaiveDateTime>,
    pub permissions: Permissions,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct User {
    pub id: i64,
    pub username: String,
    pub password: String,
    pub terminate: Option<NaiveDateTime>,
    pub permissions: Permissions,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct UserWithoutPassword {
    pub id: i64,
    pub username: String,
    pub terminate: Option<NaiveDateTime>,
    pub permissions: Permissions,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct UserPermissions {
    pub id: i64,
    pub terminate: Option<NaiveDateTime>,
    pub permissions: Permissions,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct UserIdPassword {
    pub id: i64,
    pub password: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct UserNameID {
    pub id: i64,
    pub username: String,
}
