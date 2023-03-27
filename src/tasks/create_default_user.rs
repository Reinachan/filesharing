use sqlx::{Pool, Sqlite};
use std::env::var;

use bcrypt::{hash, DEFAULT_COST};

use crate::{
    db::create_user_db,
    models::{Permissions, User},
};

pub async fn create_default_user(db: Pool<Sqlite>) {
    let username = match var("DEFAULT_USERNAME") {
        Ok(name) => name,
        Err(_) => return,
    };
    let password = match var("DEFAULT_PASSWORD") {
        Ok(pass) => hash(pass, DEFAULT_COST).unwrap(),
        Err(_) => return,
    };

    let permissions = Permissions {
        manage_users: true,
        upload_files: true,
        list_files: true,
        delete_files: true,
    };

    let user = User {
        username,
        password,
        terminate: None,
        permissions,
    };

    // Create default user
    match create_user_db(&db, user).await {
        Ok(_) => {
            println!("Default user with all permissions created");
            panic!("Make sure you remove the DEFAULT_USERNAME and DEFAULT_PASSWORD environment variables");
        }
        Err(err) => panic!("{:#?}", err),
    };
}
