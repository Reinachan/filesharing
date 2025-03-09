use axum::http::StatusCode;
use sqlx::{Pool, Sqlite};
use std::fs::remove_file;

use crate::helpers::files_path;

pub async fn delete_file(
    db: &Pool<Sqlite>,
    saved_name: String,
) -> Result<(StatusCode, String), (StatusCode, String)> {
    match remove_file(files_path(&saved_name)) {
        Ok(_) => (),
        Err(err) => match err.kind() {
            std::io::ErrorKind::NotFound => (),
            _ => {
                return Err((
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Couldn't delete".to_owned(),
                ));
            }
        },
    };

    match sqlx::query!(
        "
    DELETE FROM files WHERE saved_name = ?
    ",
        saved_name
    )
    .execute(db)
    .await
    {
        Ok(_) => Ok((StatusCode::OK, "Deleted file".to_owned())),
        Err(_) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            "Couldn't remove from database".to_owned(),
        )),
    }
}
