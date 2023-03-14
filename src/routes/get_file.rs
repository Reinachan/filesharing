use axum::{
    body::StreamBody,
    extract::{Path, State},
    http::{header, StatusCode},
    response::{AppendHeaders, IntoResponse},
};
use sqlx::{Pool, Sqlite};
use tokio::fs::File;
use tokio_util::io::ReaderStream;

use crate::{constants::ROOT_FOLDER, db::get_file_from_db};

pub async fn get_file(
    State(db): State<Pool<Sqlite>>,
    Path(file_name): Path<String>,
) -> impl IntoResponse {
    let db_file = get_file_from_db(db, file_name.clone()).await?;

    if db_file.hashed_password.is_some() {
        return Err((StatusCode::NOT_FOUND, "File not found".to_owned()));
    }

    let file = match File::open(format!("{}/{}", ROOT_FOLDER, db_file.saved_name)).await {
        Ok(res) => res,
        Err(err) => {
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("File not found: {}", err),
            ))
        }
    };

    let mime = db_file.file_type;

    let headers = AppendHeaders([(header::CONTENT_TYPE, mime)]);
    let stream = ReaderStream::new(file);
    let body = StreamBody::new(stream);

    Ok((headers, body))
}
