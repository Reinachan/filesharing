use axum::http::StatusCode;
use sqlx::{Pool, Sqlite};

use crate::models::FileDB;

pub async fn get_file_from_db(
    db: Pool<Sqlite>,
    file_name: String,
) -> Result<FileDB, (StatusCode, String)> {
    let file = match sqlx::query_as!(
        FileDB,
        "
    SELECT * FROM files WHERE saved_name=?
    ",
        file_name
    )
    .fetch_one(&db)
    .await
    {
        Ok(res) => res,
        Err(_) => return Err((StatusCode::NOT_FOUND, "File not found".to_owned())),
    };

    Ok(file)
}
