use axum::{
    body::Bytes,
    extract::{Multipart, State},
    http::StatusCode,
};
use axum_extra::{TypedHeader, headers::Cookie};
use bcrypt::{DEFAULT_COST, hash};
use sqlx::{Pool, Sqlite};
use tokio::fs::create_dir;
use uuid::Uuid;
// use futures::stream::StreamExt;
use crate::{
    constants::SERVER_DOMAIN,
    handlers::{AuthOrBasic, check_auth},
    helpers::files_path,
    models::Permissions,
};

use std::{path::Path, thread::sleep, time::Duration};

use tokio::{
    fs::{File, OpenOptions, remove_dir_all, write},
    io::{AsyncReadExt, AsyncWriteExt},
};

pub async fn upload_file(
    TypedHeader(cookie): TypedHeader<Cookie>,
    State(db): State<Pool<Sqlite>>,
    mut multipart: Multipart,
) -> Result<(StatusCode, String), (StatusCode, String)> {
    let user = check_auth(
        &db,
        AuthOrBasic::Cookie(cookie),
        Some(Permissions {
            manage_users: false,
            upload_files: true,
            list_files: false,
            delete_files: false,
        }),
    )
    .await?;

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

    match write(files_path(&saved_name), file).await {
        Ok(_) => (),
        Err(err) => return Err((StatusCode::INTERNAL_SERVER_ERROR, err.to_string())),
    };

    sqlx::query!(
        "
        INSERT INTO files (saved_name, file_name, file_type, user_id) values (?, ?, ?, ?)
        ",
        saved_name,
        name,
        data_type,
        user.id
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
        create_dir(files_path(spliced_name))
            .await
            .map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()))?;
        return Ok((StatusCode::OK, saved_name));
    }

    Ok((StatusCode::OK, format!("{}/{}", *SERVER_DOMAIN, saved_name)))
}

pub async fn put_upload_file(
    State(db): State<Pool<Sqlite>>,
    mut multipart: Multipart,
) -> Result<(StatusCode, String), (StatusCode, String)> {
    let mut filename = String::new();
    let mut file = Bytes::new();
    let mut index = 0;
    let mut finished = false;

    while let Some(field) = multipart.next_field().await.unwrap() {
        let field_name = field.name().unwrap().to_string();

        if field_name == *"chunk" {
            file = field
                .bytes()
                .await
                .map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()))?;
        } else if field_name == *"filename" {
            filename = field.text().await.unwrap();
        } else if field_name == *"index" {
            index = field.text().await.unwrap().parse::<i32>().unwrap();
        } else if field_name == *"final" {
            finished = true;
        }
    }

    let name = filename.split('.').next().unwrap();

    if !Path::new(&files_path(name)).exists() {
        return Err((
            StatusCode::BAD_REQUEST,
            "Can't write to a file that hasn't been initiated. Path doesn't exist".to_string(),
        ));
    }

    if finished {
        println!("finished");
        sleep(Duration::from_secs(1));
        concatenate_files(name, filename.clone()).await?;

        sqlx::query!(
            "
            UPDATE files 
            SET created_at = current_timestamp
            WHERE saved_name = ?
            ",
            filename,
        )
        .execute(&db)
        .await
        .map_err(|_| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Couldn't save".to_owned(),
            )
        })?;

        return Ok((StatusCode::OK, format!("{}/{}", *SERVER_DOMAIN, filename)));
    }

    if file.is_empty() {
        return Err((StatusCode::BAD_REQUEST, "No file included".to_string()));
    }

    write(format!("{}/{}-{}", files_path(name), name, index), file)
        .await
        .map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()))?;

    Ok((StatusCode::OK, format!("{}/{}", *SERVER_DOMAIN, filename)))
}

async fn concatenate_files(name: &str, filename: String) -> Result<(), (StatusCode, String)> {
    let dir_name = files_path(name);

    let number_of_files = Path::new(&dir_name).read_dir().unwrap().count();

    for index in 0..number_of_files {
        let mut file = File::open(format!("{}/{}-{}", dir_name, name, index))
            .await
            .map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()))?;

        let mut disk_file = OpenOptions::new()
            .append(true)
            .open(files_path(&filename))
            .await
            .map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()))?;

        let mut buffer = Vec::new();

        file.read_to_end(&mut buffer)
            .await
            .map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()))?;

        disk_file
            .write(&buffer)
            .await
            .map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()))?;
    }

    sleep(Duration::from_secs(1));
    remove_dir_all(dir_name)
        .await
        .map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()))?;

    Ok(())
}
