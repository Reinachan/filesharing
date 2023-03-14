use std::collections::HashMap;

use axum::{
    extract::{Multipart, Query, State},
    http::StatusCode,
    response::{Html, IntoResponse},
};
use sqlx::{Pool, Sqlite};

use crate::db::delete_file;

pub async fn delete_file_route(
    State(db): State<Pool<Sqlite>>,
    mut multipart: Multipart,
) -> impl IntoResponse {
    let mut saved_name = String::new();

    while let Some(field) = multipart.next_field().await.unwrap() {
        let field_name = field.name().unwrap().to_string();

        if field_name == *"delete" {
            saved_name = field.text().await.unwrap();
        }
    }

    let deletion = delete_file(db, saved_name).await;

    if deletion.0 == StatusCode::OK {
        return (StatusCode::OK, Html("<p>deleted</p>".to_string()));
    };

    (deletion.0, Html(format!("<p>{}</p>", deletion.1)))
}
