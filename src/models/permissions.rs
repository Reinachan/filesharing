use serde::{Deserialize, Serialize};

// We need to allow dead code because this struct is related
// to SQL and the username field exists in the database
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct PermissionsDB {
    pub username: String,
    pub manage_users: bool,
    pub upload_files: bool,
    pub list_files: bool,
    pub delete_files: bool,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Permissions {
    pub manage_users: bool,
    pub upload_files: bool,
    pub list_files: bool,
    pub delete_files: bool,
}
