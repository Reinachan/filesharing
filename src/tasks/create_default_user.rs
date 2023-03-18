use sqlx::{Pool, Sqlite};
use std::env::var;

use bcrypt::{hash, DEFAULT_COST};

use crate::models::{PermissionsDB, UserDB};

pub async fn create_default_user(db: Pool<Sqlite>) {
    let username = match var("DEFAULT_USERNAME") {
        Ok(name) => name,
        Err(_) => return,
    };
    let password = match var("DEFAULT_PASSWORD") {
        Ok(pass) => hash(pass, DEFAULT_COST).unwrap(),
        Err(_) => return,
    };

    let permissions = PermissionsDB {
        username: username.clone(),
        create_users: true,
        upload_files: true,
        list_files: true,
        delete_files: true,
    };

    let account = UserDB {
        username,
        password,
        terminate: None,
    };

    // Create default user
    match sqlx::query!(
        "
        BEGIN TRANSACTION;
        INSERT INTO users (username, password, terminate) values (?, ?, ?);
        INSERT INTO permissions (username, create_users, upload_files, list_files, delete_files) values (?, ?, ?, ?, ?);
        COMMIT;
        ",
        account.username,
        account.password,
        account.terminate,
        permissions.username,
        permissions.create_users,
        permissions.upload_files,
        permissions.list_files,
        permissions.delete_files
    )
    .execute(&db)
    .await
    {
        Ok(_) => {
            println!("Default user with all permissions created");
            panic!("Make sure you remove the DEFAULT_USERNAME and DEFAULT_PASSWORD environment variables");
        },
        Err(err) => panic!("{:#?}", err),
    };
}
