use axum::http::StatusCode;
use sqlx::{Pool, Sqlite};

use crate::models::{Permissions, PermissionsDB, User, UserDB};

pub async fn get_user_from_db(
    username: String,
    db: &Pool<Sqlite>,
) -> Result<User, (StatusCode, String)> {
    let user = sqlx::query_as!(
        UserDB,
        "
        SELECT * FROM users WHERE username = ?
        ",
        username
    )
    .fetch_one(db)
    .await
    .map_err(|_| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Cannot find a user by that username".to_string(),
        )
    })?;

    let permissions = sqlx::query_as!(
        PermissionsDB,
        "
        SELECT * FROM permissions WHERE username = ?
        ",
        username
    )
    .fetch_one(db)
    .await
    .unwrap_or(PermissionsDB {
        username: user.username.clone(),
        manage_users: false,
        upload_files: false,
        list_files: false,
        delete_files: false,
    });

    let permissions: Permissions = Permissions {
        manage_users: permissions.manage_users,
        upload_files: permissions.upload_files,
        list_files: permissions.list_files,
        delete_files: permissions.delete_files,
    };

    Ok(User {
        username: user.username,
        password: user.password,
        terminate: user.terminate,
        permissions,
    })
}
