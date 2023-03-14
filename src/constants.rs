use std::env::var;

use lazy_static::lazy_static;

pub const ROOT_FOLDER: &str = "files";

lazy_static! {
    pub static ref SERVER_DOMAIN: String =
        var("SERVER_DOMAIN").expect("Couldn't find SERVER_DOMAIN in .env file");
}
