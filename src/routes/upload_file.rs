use axum::{
    body::Bytes,
    extract::{Multipart, State},
    http::StatusCode,
};
use bcrypt::{hash, DEFAULT_COST};
use sqlx::{Pool, Sqlite};
use uuid::Uuid;
// use futures::stream::StreamExt;
use crate::constants::ROOT_FOLDER;
use std::fs::write;

pub async fn upload_file(
    State(state): State<Pool<Sqlite>>,
    mut multipart: Multipart,
) -> (StatusCode, String) {
    let mut name = String::new();
    let mut data_type = String::new();
    let mut password = String::new();
    let mut destroy = String::new();
    let mut file: Bytes = Bytes::new();

    while let Some(field) = multipart.next_field().await.unwrap() {
        let field_name = field.name().unwrap().to_string();

        if field_name == *"file" {
            let file_name = field.file_name().unwrap().to_string();
            name = file_name.clone();

            let content_type = field.content_type().unwrap().to_string();
            data_type = content_type;

            file = field.bytes().await.unwrap();
        } else if field_name == *"password" {
            let field_password = field.text().await.unwrap();
            password = hash(field_password, DEFAULT_COST).unwrap();
        } else if field_name == "destroy" {
            destroy = field.text().await.unwrap();
            println!("{destroy}");
        }
    }

    if file.is_empty() {
        return (StatusCode::BAD_REQUEST, "No file included".to_string());
    }

    if password.chars().count() < 1 {
        name = format!(
            "{}.{}",
            Uuid::new_v4(),
            match name.split('.').last() {
                Some(ext) => ext,
                None =>
                    return (
                        StatusCode::BAD_REQUEST,
                        "File lacks file extension".to_string()
                    ),
            }
        );
    }

    write(format!("{}/{}", ROOT_FOLDER, &name), file).expect("couldn't create file");

    match sqlx::query!(
        "
        INSERT INTO files (file_name, file_type) values (?, ?)
        ",
        name,
        data_type,
    )
    .execute(&state)
    .await
    {
        Ok(_) => {}
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Couldn't save".to_owned(),
            )
        }
    };

    if password.chars().count() > 0 {
        match sqlx::query!(
            "
            UPDATE files 
            SET password = ?
            WHERE file_name = ?
            ",
            password,
            name,
        )
        .execute(&state)
        .await
        {
            Ok(_) => (StatusCode::OK, format!("localhost:3000/{}", name)),
            Err(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Couldn't save".to_owned(),
            ),
        };
    };

    if destroy.chars().count() > 0 {
        match sqlx::query!(
            "
            UPDATE files 
            SET destroy = ?
            WHERE file_name = ?
            ",
            destroy,
            name,
        )
        .execute(&state)
        .await
        {
            Ok(_) => (StatusCode::OK, format!("localhost:3000/{}", name)),
            Err(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Couldn't save".to_owned(),
            ),
        };
    };

    (StatusCode::OK, format!("localhost:3000/{}", name))
}
