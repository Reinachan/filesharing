use serde::{Deserialize, Serialize};
use sqlx::types::chrono::NaiveDateTime;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileDB {
    pub saved_name: String,
    pub file_name: String,
    pub file_type: String,
    pub destroy: Option<NaiveDateTime>,
    pub password: Option<String>,
    pub user_id: i64,
    pub created_at: Option<NaiveDateTime>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct File {
    pub saved_name: String,
    pub file_name: String,
    pub file_type: String,
    pub destroy: Option<NaiveDateTime>,
    pub password: Option<String>,
    pub username: String,
    pub user_id: i64,
    pub created_at: Option<NaiveDateTime>,
}
