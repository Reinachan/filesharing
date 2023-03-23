use axum::{extract::State, headers::Cookie, http::StatusCode, response::Html, TypedHeader};
use maud::{html, DOCTYPE};
use sqlx::{Pool, Sqlite};

use crate::{
    handlers::{check_auth, AuthOrBasic},
    models::Permissions,
};

pub async fn profile(
    TypedHeader(cookie): TypedHeader<Cookie>,
    State(db): State<Pool<Sqlite>>,
) -> Result<Html<String>, (StatusCode, String)> {
    let user = check_auth(
        &db,
        AuthOrBasic::Cookie(cookie),
        Some(Permissions {
            create_users: false,
            upload_files: false,
            list_files: false,
            delete_files: false,
        }),
    )
    .await?;

    let permissions = vec![
        ("Manage users", user.permissions.create_users),
        ("Upload files", user.permissions.upload_files),
        ("List files", user.permissions.list_files),
        ("Delete files", user.permissions.delete_files),
    ];

    Ok(Html(
        html! {
            (DOCTYPE)
            html {
                head {
                    title { "Filehost" }
                    link rel="stylesheet" type="text/css" href="assets/styles.css";
                    meta name="viewport" content="width=device-width, initial-scale=1.0";
                }
                body {
                    nav {
                        ul {
                            li { a href="/" { "home" }}
                            li { a href="/upload" { "upload" }}
                            li { a href="/files" { "files list" }}
                            li { a class="current" href="/profile" { (user.username) }}
                        }
                    }
                    h2 { (user.username) }
                    h3 { "Permissions" }
                    ul {
                        @for permission in permissions {
                            li {
                                (permission.0)
                                @if permission.1 {
                                    input type="checkbox" checked disabled;
                                } @else {
                                    input type="checkbox" disabled;
                                }
                            }
                        }
                    }
                }
            }
        }
        .into_string(),
    ))
}
