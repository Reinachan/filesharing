use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use sqlx::{Pool, Sqlite};
use std::fs::remove_file;

use crate::constants::ROOT_FOLDER;

pub async fn delete_file(
    Path(payload): Path<String>,
    State(db): State<Pool<Sqlite>>,
) -> impl IntoResponse {
    match remove_file(format!("{}/{}", ROOT_FOLDER, payload)) {
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
    DELETE FROM files WHERE file_name = ?
    ",
        payload
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
