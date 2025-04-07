use axum::{
    body::Body,
    extract::{Multipart, State},
    http::{StatusCode, header},
    response::{AppendHeaders, IntoResponse},
};
use bcrypt::verify;
use sqlx::{Pool, Sqlite};
use tokio::fs::File;
use tokio_util::io::ReaderStream;

use crate::{db::get_file_from_db, helpers::files_path};

pub async fn download_file(
    State(db): State<Pool<Sqlite>>,
    mut multipart: Multipart,
) -> impl IntoResponse {
    let mut file_name = String::new();
    let mut password = String::new();

    while let Some(field) = multipart.next_field().await.unwrap() {
        let field_name = field.name().unwrap().to_string();

        if field_name == *"file_name" {
            file_name = field.text().await.unwrap().to_string();
        } else if field_name == *"password" {
            password = field.text().await.unwrap();
        }
    }

    let db_file = match get_file_from_db(db, file_name.clone()).await {
        Ok(value) => value,
        Err(err) => {
            // this verification will always fail. Prevents filename guessing.
            verify(
                "never",
                "$2b$12$wAfhk5/7W7mjQ/bJf/N.K..3F3YNN2r.Wkmq2JZLzKqRzVQIeOfwK",
            )
            .ok();
            return Err(err);
        }
    };

    if db_file.password.is_some() {
        let verified = match verify(password, &db_file.password.unwrap()) {
            Ok(value) => value,
            Err(err) => {
                return Err((
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!("Error with password hash verification: {}", err),
                ));
            }
        };

        if !verified {
            return Err((StatusCode::NOT_FOUND, "File not found".to_owned()));
        }
    }

    let file = match File::open(files_path(&db_file.saved_name)).await {
        Ok(res) => res,
        Err(err) => return Err((StatusCode::NOT_FOUND, format!("File not found: {}", err))),
    };

    let mime = db_file.file_type;

    let headers = AppendHeaders([
        (header::CONTENT_TYPE, mime),
        (
            header::CONTENT_DISPOSITION,
            format!(r#"attachment; filename="{}""#, db_file.file_name),
        ),
    ]);
    let stream = ReaderStream::new(file);
    let body = Body::from_stream(stream);

    Ok((headers, body))
}
