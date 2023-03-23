use axum::{
    body::Bytes,
    extract::{Multipart, State},
    headers::Cookie,
    http::StatusCode,
    TypedHeader,
};
use bcrypt::{hash, DEFAULT_COST};
use sqlx::{Pool, Sqlite};
use tokio::fs::create_dir;
use uuid::Uuid;
// use futures::stream::StreamExt;
use crate::{
    constants::{ROOT_FOLDER, SERVER_DOMAIN},
    handlers::{check_auth, AuthOrBasic},
    models::Permissions,
};
use std::fs::write;

pub async fn upload_file(
    TypedHeader(cookie): TypedHeader<Cookie>,
    State(db): State<Pool<Sqlite>>,
    mut multipart: Multipart,
) -> Result<(StatusCode, String), (StatusCode, String)> {
    let _user = match check_auth(
        &db,
        AuthOrBasic::Cookie(cookie),
        Some(Permissions {
            create_users: false,
            upload_files: true,
            list_files: false,
            delete_files: false,
        }),
    )
    .await
    {
        Ok(val) => val,
        Err(err) => return Err(err),
    };

    let mut name = String::new();
    let mut data_type = String::new();
    let mut password = String::new();
    let mut destroy = String::new();
    let mut file: Bytes = Bytes::new();
    let mut chunk = false;

    while let Some(field) = multipart.next_field().await.unwrap() {
        let field_name = field.name().unwrap().to_string();

        if field_name == *"file" {
            let file_name = field.file_name().unwrap().to_string();
            name = file_name.clone();

            let content_type = field.content_type().unwrap().to_string();
            data_type = content_type;

            file = field.bytes().await.unwrap();
        } else if field_name == *"filename" {
            name = field.text().await.unwrap();
            chunk = true;
        } else if field_name == *"mime" {
            data_type = field.text().await.unwrap();
        } else if field_name == *"password" {
            let field_password = field.text().await.unwrap();
            if field_password.chars().count() > 0 {
                password = hash(field_password, DEFAULT_COST).unwrap();
            }
        } else if field_name == "destroy" {
            destroy = field.text().await.unwrap();
            // println!("{destroy}");
        }
    }

    if file.is_empty() && !chunk {
        return Err((StatusCode::BAD_REQUEST, "No file included".to_string()));
    }

    let saved_name = format!(
        "{}.{}",
        Uuid::new_v4(),
        match name.split('.').last() {
            Some(ext) => ext,
            None =>
                return Err((
                    StatusCode::BAD_REQUEST,
                    "File lacks file extension".to_string()
                )),
        }
    );

    match write(format!("{}/{}", ROOT_FOLDER, &saved_name), file) {
        Ok(_) => (),
        Err(err) => return Err((StatusCode::INTERNAL_SERVER_ERROR, err.to_string())),
    };

    sqlx::query!(
        "
        INSERT INTO files (saved_name, file_name, file_type) values (?, ?, ?)
        ",
        saved_name,
        name,
        data_type,
    )
    .execute(&db)
    .await
    .map_err(|_| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Couldn't save".to_owned(),
        )
    })?;

    if password.chars().count() > 0 {
        sqlx::query!(
            "
            UPDATE files 
            SET password = ?
            WHERE saved_name = ?
            ",
            password,
            saved_name,
        )
        .execute(&db)
        .await
        .map_err(|_| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Couldn't save".to_owned(),
            )
        })?;
    };

    if destroy.chars().count() > 0 {
        sqlx::query!(
            "
            UPDATE files 
            SET destroy = ?
            WHERE saved_name = ?
            ",
            destroy,
            saved_name,
        )
        .execute(&db)
        .await
        .map_err(|_| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Couldn't save".to_owned(),
            )
        })?;
    };

    if chunk {
        let spliced_name = saved_name.split('.').next().unwrap();
        create_dir(format!("{}/{}", ROOT_FOLDER, spliced_name))
            .await
            .map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()))?;
        return Ok((StatusCode::OK, saved_name));
    }

    Ok((StatusCode::OK, format!("{}/{}", *SERVER_DOMAIN, saved_name)))
}
