use axum::http::StatusCode;
use sqlx::{Pool, Sqlite};

use crate::models::{Permissions, PermissionsDB, User, UserDB};

pub async fn get_users_from_db(db: Pool<Sqlite>) -> Result<Vec<User>, (StatusCode, String)> {
    let users = match sqlx::query_as!(
        UserDB,
        "
        SELECT * FROM users
        "
    )
    .fetch_all(&db)
    .await
    {
        Ok(all_users) => all_users,
        Err(err) => return Err((StatusCode::NOT_FOUND, format!("File not found: {}", err))),
    };

    let mut all_users: Vec<User> = Vec::new();

    for user in users.iter() {
        let id = user.id;

        let permissions = sqlx::query_as!(
            PermissionsDB,
            "
    SELECT * FROM permissions WHERE id = ?
    ",
            id
        )
        .fetch_one(&db)
        .await
        .unwrap_or(PermissionsDB {
            id: user.id,
            manage_users: false,
            upload_files: false,
            list_files: false,
            delete_files: false,
        });

        all_users.push(User {
            id: user.id,
            username: user.username.clone(),
            password: user.password.clone(),
            terminate: user.terminate,
            permissions: Permissions {
                manage_users: permissions.manage_users,
                upload_files: permissions.upload_files,
                list_files: permissions.list_files,
                delete_files: permissions.delete_files,
            },
        });
    }

    Ok(all_users)
}
