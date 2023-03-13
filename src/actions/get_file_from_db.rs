use axum::http::StatusCode;
use sqlx::{Pool, Sqlite};

use crate::models::{File, FileWithPassword};

pub async fn get_file_from_db(
    db: Pool<Sqlite>,
    file_name: String,
) -> Result<FileWithPassword, (StatusCode, String)> {
    let file: FileWithPassword = match sqlx::query!(
        "
    SELECT * FROM files WHERE file_name=?
    ",
        file_name
    )
    .fetch_one(&db)
    .await
    {
        Ok(res) => FileWithPassword {
            file_name: res.file_name,
            file_type: res.file_type,
            destroy: res.destroy.map(|destroy| destroy.timestamp()),
            hashed_password: res.password,
        },
        Err(_) => return Err((StatusCode::NOT_FOUND, "File not found".to_owned())),
    };

    Ok(file)
}
