use sqlx::types::chrono::NaiveDateTime;

#[derive(Debug, Clone)]
pub struct FileDB {
    pub saved_name: String,
    pub file_name: String,
    pub file_type: String,
    pub destroy: Option<NaiveDateTime>,
    pub password: Option<String>,
}
