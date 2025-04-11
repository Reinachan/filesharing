use axum::{Extension, Json, extract::State, http::StatusCode, response::IntoResponse};
use serde::Serialize;
use sqlx::{Pool, Sqlite};

use crate::{
    db::get_own_files_from_db,
    models::{File, User},
};

#[derive(Serialize)]
struct Response {
    files: Vec<File>,
}

pub async fn own_files(
    State(db): State<Pool<Sqlite>>,
    Extension(user): Extension<User>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    Ok(Json(Response {
        files: get_own_files_from_db(db, user.id).await?,
    }))
}
