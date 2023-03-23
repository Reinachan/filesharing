use axum::{
    extract::{Multipart, State},
    headers::Cookie,
    http::StatusCode,
    response::Redirect,
    TypedHeader,
};
use sqlx::{Pool, Sqlite};

use crate::{
    db::delete_file,
    handlers::{check_auth, AuthOrBasic},
    models::Permissions,
};

pub async fn delete_file_route(
    TypedHeader(cookie): TypedHeader<Cookie>,
    State(db): State<Pool<Sqlite>>,
    mut multipart: Multipart,
) -> Result<Redirect, (StatusCode, String)> {
    let _user = match check_auth(
        &db,
        AuthOrBasic::Cookie(cookie),
        Some(Permissions {
            create_users: false,
            upload_files: false,
            list_files: false,
            delete_files: true,
        }),
    )
    .await
    {
        Ok(val) => val,
        Err(err) => return Err(err),
    };

    let mut saved_name = String::new();

    while let Some(field) = multipart.next_field().await.unwrap() {
        let field_name = field.name().unwrap().to_string();

        if field_name == *"delete" {
            saved_name = field.text().await.unwrap();
        }
    }

    let deletion = delete_file(&db, saved_name).await?;

    if deletion.0 == StatusCode::OK {
        return Ok(Redirect::to("/files"));
    };

    Ok(Redirect::to("/files"))
}
