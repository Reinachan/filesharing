use axum::http::StatusCode;
use sqlx::{Pool, Sqlite};

use crate::models::File;

pub async fn get_file_from_db(
    db: &Pool<Sqlite>,
    file_name: &String,
) -> Result<File, (StatusCode, String)> {
    let file = match sqlx::query_as!(
        File,
        "
        select saved_name, file_name, file_type, files.password, destroy, user_id, username, created_at
        from files
        inner join users on files.user_id = users.id
        where saved_name=?
        ",
        file_name
    )
    .fetch_one(db)
    .await
    {
        Ok(res) => res,
        Err(_) => return Err((StatusCode::NOT_FOUND, "File not found".to_owned())),
    };

    Ok(file)
}
