use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct File {
    pub file_name: String,
    pub file_type: String,
    pub destroy: Option<i64>,
    pub password_protected: bool,
}

#[derive(Serialize, Deserialize)]
pub struct FileWithPassword {
    pub file_name: String,
    pub file_type: String,
    pub destroy: Option<i64>,
    pub hashed_password: Option<String>,
}
