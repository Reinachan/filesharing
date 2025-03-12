use axum::{Extension, Json, extract::State, http::StatusCode, response::IntoResponse};
use serde::Deserialize;
use sqlx::{Pool, Sqlite};

use crate::{db::delete_file, models::User};

#[derive(Deserialize)]
pub struct FileToDelete {
    filename: String,
}

pub async fn admin_files(
    State(db): State<Pool<Sqlite>>,
    Extension(user): Extension<User>,
    Json(file): Json<FileToDelete>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    match user.permissions.delete_files {
        true => {
            delete_file(&db, file.filename).await?;
            Ok(StatusCode::NO_CONTENT)
        }
        false => Err((
            StatusCode::FORBIDDEN,
            "You don't have permission to fetch this resource".to_string(),
        )),
    }
}
