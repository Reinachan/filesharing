use std::env::var;

use lazy_static::lazy_static;

pub const AUTH_COOKIE: &str = "Authorization";

lazy_static! {
    pub static ref SERVER_PORT: String = var("SERVER_PORT").unwrap_or("9800".to_string());
    pub static ref SERVER_DOMAIN: String =
        var("SERVER_DOMAIN").unwrap_or(format!("http://localhost:{}", *SERVER_PORT));
    pub static ref SERVER_NAME: String = var("SERVER_NAME").unwrap_or("Filehost".to_string());
    pub static ref ROOT_FOLDER: String = var("ROOT_FOLDER").unwrap_or("files".to_string());
}
