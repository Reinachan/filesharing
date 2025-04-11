use axum::{extract::State, http::StatusCode, response::Html};
use axum_extra::{TypedHeader, headers::Cookie};
use maud::{DOCTYPE, html};
use sqlx::{Pool, Sqlite};

use crate::{
    handlers::{AuthOrBasic, check_auth},
    models::Permissions,
    views::templates::{Routes, head, nav},
};

pub async fn profile(
    TypedHeader(cookie): TypedHeader<Cookie>,
    State(db): State<Pool<Sqlite>>,
) -> Result<Html<String>, (StatusCode, String)> {
    let user = check_auth(
        &db,
        AuthOrBasic::Cookie(cookie),
        Some(Permissions {
            manage_users: false,
            upload_files: false,
            list_files: false,
            delete_files: false,
        }),
    )
    .await?;

    let permissions = vec![
        ("Manage users", user.permissions.manage_users),
        ("Upload files", user.permissions.upload_files),
        ("List files", user.permissions.list_files),
        ("Delete files", user.permissions.delete_files),
    ];

    Ok(Html(
        html! {
            (DOCTYPE)
            html {
                (head(&user.username, None, None))
                body {
                    (nav(Routes::Profile, Some(&user.username), Some(user.permissions)))
                    h2 { (user.username) }
                    p { "User ID: " (user.id) }
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
