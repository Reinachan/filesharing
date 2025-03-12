use axum::{Extension, Json, extract::State, http::StatusCode, response::IntoResponse};
use serde::Serialize;
use sqlx::{Pool, Sqlite};

use crate::{
    db::get_files_from_db,
    models::{FileDB, User},
};

#[derive(Serialize)]
struct Response {
    files: Vec<FileDB>,
}

pub async fn admin_files(
    State(db): State<Pool<Sqlite>>,
    Extension(user): Extension<User>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    match user.permissions.list_files {
        true => Ok(Json(Response {
            files: get_files_from_db(db).await?,
        })),
        false => Err((
            StatusCode::FORBIDDEN,
            "You don't have permission to fetch this resource".to_string(),
        )),
    }
}
