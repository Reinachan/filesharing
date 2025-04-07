use axum::{
    extract::{Multipart, State},
    http::StatusCode,
    response::Redirect,
};
use axum_extra::{TypedHeader, headers::Cookie};
use bcrypt::{DEFAULT_COST, hash};
use sqlx::{Pool, Sqlite, types::chrono::NaiveDateTime};
// use futures::stream::StreamExt;
use crate::{
    db::{create_user_db, delete_user_db, edit_user_db},
    handlers::{AuthOrBasic, check_auth},
    helpers::link_path,
    models::{Permissions, User},
};

pub async fn create_user(
    TypedHeader(cookie): TypedHeader<Cookie>,
    State(db): State<Pool<Sqlite>>,
    mut multipart: Multipart,
) -> Result<Redirect, (StatusCode, String)> {
    let _user = check_auth(
        &db,
        AuthOrBasic::Cookie(cookie),
        Some(Permissions {
            manage_users: true,
            upload_files: false,
            list_files: false,
            delete_files: false,
        }),
    )
    .await?;

    let mut username = String::new();
    let mut password = String::new();
    let mut terminate: Option<NaiveDateTime> = None;
    let mut permissions = Permissions::default();

    while let Some(field) = multipart.next_field().await.unwrap() {
        let field_name = field.name().unwrap().to_string();

        if field_name == *"username" {
            username = field.text().await.unwrap();
        } else if field_name == *"password" {
            let field_password = field.text().await.unwrap();
            if field_password.chars().count() > 0 {
                password = hash(field_password, DEFAULT_COST).unwrap();
            }
        } else if field_name == *"terminate" {
            let inputted_date = &field
                .text()
                .await
                .map_err(|err| (StatusCode::BAD_REQUEST, err.to_string()))?;

            if !inputted_date.is_empty() {
                terminate = Some(
                    NaiveDateTime::parse_from_str(inputted_date, "%Y-%m-%dT%H:%M")
                        .map_err(|err| (StatusCode::BAD_REQUEST, err.to_string()))?,
                );
            }
        } else if field_name == *"manage_users" {
            permissions.manage_users = true;
        } else if field_name == *"upload_files" {
            permissions.upload_files = true;
        } else if field_name == *"list_files" {
            permissions.list_files = true;
        } else if field_name == *"delete_files" {
            permissions.delete_files = true;
        }
    }

    create_user_db(
        &db,
        User {
            username,
            password,
            terminate,
            permissions,
        },
    )
    .await?;

    Ok(Redirect::to(&link_path("/users")))
}

pub async fn delete_user(
    TypedHeader(cookie): TypedHeader<Cookie>,
    State(db): State<Pool<Sqlite>>,
    mut multipart: Multipart,
) -> Result<Redirect, (StatusCode, String)> {
    let _user = check_auth(
        &db,
        AuthOrBasic::Cookie(cookie),
        Some(Permissions {
            manage_users: true,
            upload_files: false,
            list_files: false,
            delete_files: false,
        }),
    )
    .await?;

    let mut username = String::new();

    while let Some(field) = multipart.next_field().await.unwrap() {
        let field_name = field.name().unwrap().to_string();

        if field_name == *"username" {
            username = field.text().await.unwrap();
        }
    }

    delete_user_db(&db, username).await?;

    Ok(Redirect::to(&link_path("/users")))
}

pub async fn edit_user(
    TypedHeader(cookie): TypedHeader<Cookie>,
    State(db): State<Pool<Sqlite>>,
    mut multipart: Multipart,
) -> Result<Redirect, (StatusCode, String)> {
    let _user = check_auth(
        &db,
        AuthOrBasic::Cookie(cookie),
        Some(Permissions {
            manage_users: true,
            upload_files: false,
            list_files: false,
            delete_files: false,
        }),
    )
    .await?;

    let mut username = String::new();
    let mut password = String::new();
    let mut terminate: Option<NaiveDateTime> = None;
    let mut permissions = Permissions::default();

    while let Some(field) = multipart.next_field().await.unwrap() {
        let field_name = field.name().unwrap().to_string();

        if field_name == *"username" {
            username = field.text().await.unwrap();
        } else if field_name == *"password" {
            let field_password = field.text().await.unwrap();
            if field_password.chars().count() > 0 {
                password = hash(field_password, DEFAULT_COST).unwrap();
            }
        } else if field_name == *"terminate" {
            terminate = Some(
                NaiveDateTime::parse_from_str(&field.text().await.unwrap(), "%Y-%m-%dT%H:%M")
                    .map_err(|err| (StatusCode::BAD_REQUEST, err.to_string()))?,
            );
        } else if field_name == *"manage_users" {
            permissions.manage_users = true;
        } else if field_name == *"upload_files" {
            permissions.upload_files = true;
        } else if field_name == *"list_files" {
            permissions.list_files = true;
        } else if field_name == *"delete_files" {
            permissions.delete_files = true;
        }
    }

    edit_user_db(
        &db,
        User {
            username,
            password,
            terminate,
            permissions,
        },
    )
    .await?;

    Ok(Redirect::to(&link_path("/users")))
}
