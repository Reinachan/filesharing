use axum::http::StatusCode;
use sqlx::{Pool, Sqlite};

use crate::models::File;

pub async fn get_files_from_db(db: Pool<Sqlite>) -> Result<Vec<File>, (StatusCode, String)> {
    let files: Vec<File> = match sqlx::query!(
        "
    SELECT * FROM files
    "
    )
    .fetch_all(&db)
    .await
    {
        Ok(all_files) => {
            let test = all_files
                .iter()
                .map(|file| File {
                    file_name: file.file_name.to_owned(),
                    file_type: file.file_type.to_owned(),
                    destroy: file.destroy.map(|destroy| destroy.timestamp()),
                    password_protected: file.password.is_some(),
                })
                .collect();
            test
        }
        Err(err) => return Err((StatusCode::NOT_FOUND, format!("File not found: {}", err))),
    };

    Ok(files)
}
