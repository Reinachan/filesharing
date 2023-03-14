use axum::http::StatusCode;
use sqlx::{Pool, Sqlite};
use std::fs::remove_file;

use crate::constants::ROOT_FOLDER;

pub async fn delete_file(db: Pool<Sqlite>, saved_name: String) -> (StatusCode, String) {
    match remove_file(format!("{}/{}", ROOT_FOLDER, saved_name)) {
        Ok(_) => (),
        Err(err) => match err.kind() {
            std::io::ErrorKind::NotFound => (),
            _ => {
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Couldn't delete".to_owned(),
                )
            }
        },
    };

    match sqlx::query!(
        "
    DELETE FROM files WHERE saved_name = ?
    ",
        saved_name
    )
    .fetch_all(&db)
    .await
    {
        Ok(_) => (StatusCode::OK, "Deleted file".to_owned()),
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Couldn't remove from database".to_owned(),
        ),
    }
}
