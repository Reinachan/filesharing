use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct File {
    pub saved_name: String,
    pub file_name: String,
    pub file_type: String,
    pub destroy: Option<i64>,
    pub password_protected: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FileWithPassword {
    pub saved_name: String,
    pub file_name: String,
    pub file_type: String,
    pub destroy: Option<i64>,
    pub hashed_password: Option<String>,
}
