use axum::http::StatusCode;
use sqlx::{Pool, Sqlite};

use crate::models::UsernamePassword;

pub async fn edit_user_password(
    db: &Pool<Sqlite>,
    user: UsernamePassword,
) -> Result<(StatusCode, String), (StatusCode, String)> {
    let query = sqlx::query!(
        "
        BEGIN TRANSACTION;
        
        UPDATE users 
        SET password = ?
        WHERE id = ?;
        
        COMMIT;
        ",
        user.password,
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

    // FIXME: This method seems a little inconsistent at figuring out if the entry was updated
    //        look into a better way of returning an error if a user doesn't exist
    if query.rows_affected() <= 1 {
        return Err((StatusCode::NOT_FOUND, "User does not exist".to_owned()));
    }

    Ok((StatusCode::OK, "Updated password".to_owned()))
}
