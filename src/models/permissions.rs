#[derive(Debug, Clone)]
pub struct PermissionsDB {
    pub username: String,
    pub manage_users: bool,
    pub upload_files: bool,
    pub list_files: bool,
    pub delete_files: bool,
}

#[derive(Debug, Clone, Default)]
pub struct Permissions {
    pub manage_users: bool,
    pub upload_files: bool,
    pub list_files: bool,
    pub delete_files: bool,
}
