use axum::http::StatusCode;
use sqlx::{Pool, Sqlite};

use crate::models::User;

pub async fn create_user_db(
    db: &Pool<Sqlite>,
    user: User,
) -> Result<(StatusCode, String), (StatusCode, String)> {
    sqlx::query!(
        "
        BEGIN TRANSACTION;
        INSERT INTO users (username, password, terminate) values (?, ?, ?);
        INSERT INTO permissions (username, manage_users, upload_files, list_files, delete_files) values (?, ?, ?, ?, ?);
        COMMIT;
        ",
        user.username,
        user.password,
        user.terminate,
        user.username,
        user.permissions.manage_users,
        user.permissions.upload_files,
        user.permissions.list_files,
        user.permissions.delete_files
    )
    .execute(db)
    .await
    .map_err(|_| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Couldn't create user".to_owned(),
        )
    })?;

    Ok((StatusCode::OK, "Created user".to_owned()))
}
