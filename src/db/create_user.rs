use axum::http::StatusCode;
use sqlx::{Pool, Sqlite};

use crate::models::{CreateUserDB, User};

use super::get_user_by_username;

pub async fn create_user_db(
    db: &Pool<Sqlite>,
    user: CreateUserDB,
) -> Result<User, (StatusCode, String)> {
    sqlx::query!(
        "
        BEGIN TRANSACTION;
        INSERT INTO users (username, password, terminate) values (?, ?, ?);
        INSERT INTO permissions (id, manage_users, upload_files, list_files, delete_files) values (last_insert_rowid(), ?, ?, ?, ?);
        COMMIT;
        ",
        user.username,
        user.password,
        user.terminate,
        user.permissions.manage_users,
        user.permissions.upload_files,
        user.permissions.list_files,
        user.permissions.delete_files
    )
    .execute(db)
    .await
    .map_err(|err| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Couldn't create user, {}", err)
        )
    })?;

    get_user_by_username(&user.username, db)
        .await
        .map_err(|err| {
            (
                err,
                "Somehow we can't find the user you just created?".to_string(),
            )
        })
}
