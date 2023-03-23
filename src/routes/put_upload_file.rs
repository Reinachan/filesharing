use std::{path::Path, thread::sleep, time::Duration};

use axum::{body::Bytes, extract::Multipart, http::StatusCode};
use tokio::{
    fs::{remove_dir_all, write, File, OpenOptions},
    io::{AsyncReadExt, AsyncWriteExt},
};

use crate::constants::{ROOT_FOLDER, SERVER_DOMAIN};

pub async fn put_upload_file(
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

    if !Path::new(&format!("{}/{}", ROOT_FOLDER, name)).exists() {
        return Err((
            StatusCode::BAD_REQUEST,
            "Can't write to a file that hasn't been initiated. Path doesn't exist".to_string(),
        ));
    }

    if finished {
        println!("finished");
        sleep(Duration::from_secs(1));
        concatenate_files(name, filename.clone()).await?;
        return Ok((StatusCode::OK, format!("{}/{}", *SERVER_DOMAIN, filename)));
    }

    if file.is_empty() {
        return Err((StatusCode::BAD_REQUEST, "No file included".to_string()));
    }

    write(format!("{}/{}/{}-{}", ROOT_FOLDER, name, name, index), file)
        .await
        .map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()))?;

    Ok((StatusCode::OK, format!("{}/{}", *SERVER_DOMAIN, filename)))
}

pub async fn concatenate_files(name: &str, filename: String) -> Result<(), (StatusCode, String)> {
    let dir_name = format!("{}/{}", ROOT_FOLDER, name);

    let number_of_files = Path::new(&dir_name).read_dir().unwrap().count();

    for index in 0..number_of_files {
        let mut file = File::open(format!("{}/{}-{}", dir_name, name, index))
            .await
            .map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()))?;

        let mut disk_file = OpenOptions::new()
            .append(true)
            .open(format!("{}/{}", ROOT_FOLDER, filename))
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
