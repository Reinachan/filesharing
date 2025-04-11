use axum::http::StatusCode;
use sqlx::{Pool, Sqlite};

use crate::models::User;

pub async fn edit_user_db(
    db: &Pool<Sqlite>,
    user: User,
) -> Result<(StatusCode, String), (StatusCode, String)> {
    sqlx::query!(
        "
        BEGIN TRANSACTION;
        
        UPDATE users 
        SET password = ?, terminate = ? 
        WHERE id = ?;
        
        UPDATE permissions
        SET manage_users = ?, upload_files = ?, list_files = ?, delete_files = ?
        WHERE id = ?;
        
        COMMIT;
        ",
        user.password,
        user.terminate,
        user.username,
        user.permissions.manage_users,
        user.permissions.upload_files,
        user.permissions.list_files,
        user.permissions.delete_files,
        user.id,
    )
    .execute(db)
    .await
    .map_err(|_| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Couldn't edit user".to_owned(),
        )
    })?;

    Ok((StatusCode::OK, "Created user".to_owned()))
}
