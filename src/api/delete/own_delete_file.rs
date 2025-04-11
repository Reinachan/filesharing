use axum::{Extension, Json, extract::State, http::StatusCode, response::IntoResponse};
use serde::Deserialize;
use sqlx::{Pool, Sqlite};

use crate::{
    db::{delete_file, get_file_from_db},
    models::User,
};

#[derive(Deserialize)]
pub struct FileToDelete {
    filename: String,
}

pub async fn own_files(
    State(db): State<Pool<Sqlite>>,
    Extension(user): Extension<User>,
    Json(file): Json<FileToDelete>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let db_file = get_file_from_db(&db, &file.filename).await?;

    match user.id == db_file.user_id {
        true => {
            delete_file(&db, file.filename).await?;
            Ok(StatusCode::NO_CONTENT)
        }
        false => Err((
            StatusCode::FORBIDDEN,
            "You don't have permission to delete this resource".to_string(),
        )),
    }
}
