#[derive(Debug, Clone)]
pub struct PermissionsDB {
    pub username: String,
    pub create_users: bool,
    pub upload_files: bool,
    pub list_files: bool,
    pub delete_files: bool,
}

#[derive(Debug, Clone)]
pub struct Permissions {
    pub create_users: bool,
    pub upload_files: bool,
    pub list_files: bool,
    pub delete_files: bool,
}
