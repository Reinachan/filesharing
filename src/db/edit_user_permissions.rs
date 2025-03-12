use axum::http::StatusCode;
use sqlx::{Pool, Sqlite};

use crate::models::UserWithoutPassword;

pub async fn edit_user_permissions(
    db: &Pool<Sqlite>,
    user: &UserWithoutPassword,
) -> Result<(StatusCode, String), (StatusCode, String)> {
    let query = sqlx::query!(
        "
        BEGIN TRANSACTION;
        
        UPDATE users
        SET terminate = ?
        WHERE username = ?;
        
        UPDATE permissions
        SET manage_users = ?, upload_files = ?, list_files = ?, delete_files = ?
        WHERE username = ?;
        
        COMMIT;
        ",
        user.terminate,
        user.username,
        user.permissions.manage_users,
        user.permissions.upload_files,
        user.permissions.list_files,
        user.permissions.delete_files,
        user.username,
    )
    .execute(db)
    .await
    .map_err(|_| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Couldn't edit user".to_owned(),
        )
    })?;

    // FIXME: This method seems a little inconsistent at figuring out if the entry was updated
    //        look into a better way of returning an error if a user doesn't exist
    if query.rows_affected() <= 1 {
        return Err((StatusCode::NOT_FOUND, "User does not exist".to_owned()));
    }

    Ok((StatusCode::OK, "Updated user permissions".to_owned()))
}
