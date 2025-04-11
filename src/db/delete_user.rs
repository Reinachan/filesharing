use axum::http::StatusCode;
use sqlx::{Pool, Sqlite};

pub async fn delete_user_db(
    db: &Pool<Sqlite>,
    id: i64,
) -> Result<(StatusCode, String), (StatusCode, String)> {
    let query = sqlx::query!(
        "
        BEGIN TRANSACTION;
        DELETE FROM permissions WHERE id = ?;
        DELETE FROM users WHERE id = ?;
        COMMIT;
        ",
        id,
        id
    )
    .execute(db)
    .await
    .map_err(|_| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Couldn't remove from database".to_owned(),
        )
    })?;

    // FIXME: This method seems a little inconsistent at figuring out if the entry was updated
    //        look into a better way of returning an error if a user doesn't exist
    if query.rows_affected() <= 1 {
        return Err((StatusCode::NOT_FOUND, "User does not exist".to_owned()));
    }

    Ok((StatusCode::OK, format!("Deleted user: {}", id)))
}
