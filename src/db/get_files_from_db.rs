use axum::http::StatusCode;
use sqlx::{Pool, Sqlite};

use crate::models::File;

pub async fn get_files_from_db(db: Pool<Sqlite>) -> Result<Vec<File>, (StatusCode, String)> {
    let files = match sqlx::query_as!(
        File,
        "
        select saved_name, file_name, file_type, files.password, destroy, user_id, username, created_at
from files
inner join users on files.user_id = users.id
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
