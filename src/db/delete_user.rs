use axum::http::StatusCode;
use sqlx::{Pool, Sqlite};

pub async fn delete_user_db(
    db: &Pool<Sqlite>,
    username: String,
) -> Result<(StatusCode, String), (StatusCode, String)> {
    match sqlx::query!(
        "
        BEGIN TRANSACTION;
        DELETE FROM permissions WHERE username = ?;
        DELETE FROM users WHERE username = ?;
        COMMIT;
        ",
        username,
        username
    )
    .execute(db)
    .await
    {
        Ok(_) => Ok((StatusCode::OK, format!("Deleted user: {}", username))),
        Err(_) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            "Couldn't remove from database".to_owned(),
        )),
    }
}
