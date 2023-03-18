use axum::http::StatusCode;
use sqlx::{Pool, Sqlite};

use crate::models::FileDB;

pub async fn get_files_from_db(db: Pool<Sqlite>) -> Result<Vec<FileDB>, (StatusCode, String)> {
    let files = match sqlx::query_as!(
        FileDB,
        "
    SELECT * FROM files
    "
    )
    .fetch_all(&db)
    .await
    {
        Ok(all_files) => all_files,
        Err(err) => return Err((StatusCode::NOT_FOUND, format!("File not found: {}", err))),
    };

    Ok(files)
}
