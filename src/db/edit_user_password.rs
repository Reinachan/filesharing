use axum::http::StatusCode;
use sqlx::{Pool, Sqlite};

use crate::models::UsernamePassword;

pub async fn edit_user_password(
    db: &Pool<Sqlite>,
    user: UsernamePassword,
) -> Result<(StatusCode, String), (StatusCode, String)> {
    sqlx::query!(
        "
        BEGIN TRANSACTION;
        
        UPDATE users 
        SET password = ?
        WHERE username = ?;
        
        COMMIT;
        ",
        user.password,
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

    Ok((StatusCode::OK, "Updated password".to_owned()))
}
