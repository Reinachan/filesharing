use crate::actions::get_files_from_db;
use axum::{extract::State, response::IntoResponse, Json};
use serde::Serialize;
use sqlx::{Pool, Sqlite};

#[derive(Serialize)]
struct File {
    file_name: String,
    file_type: String,
    destroy: Option<i64>,
    password: bool,
}

pub async fn get_all_files(State(db): State<Pool<Sqlite>>) -> impl IntoResponse {
    match get_files_from_db(db).await {
        Ok(files) => Ok(Json(files)),
        Err(err) => Err(err),
    }
}
