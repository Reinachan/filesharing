use serde::{Deserialize, Serialize};
use sqlx::types::chrono::NaiveDateTime;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileDB {
    pub saved_name: String,
    pub file_name: String,
    pub file_type: String,
    pub destroy: Option<NaiveDateTime>,
    pub password: Option<String>,
}
