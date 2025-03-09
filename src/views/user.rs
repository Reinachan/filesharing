use axum::{extract::State, http::StatusCode, response::Html};
use axum_extra::{TypedHeader, headers::Cookie};
use maud::{DOCTYPE, html};
use sqlx::{Pool, Sqlite};

use crate::{
    handlers::{AuthOrBasic, check_auth},
    models::Permissions,
    views::templates::{Routes, head, nav},
};

pub async fn new_user(
    TypedHeader(cookie): TypedHeader<Cookie>,
    State(db): State<Pool<Sqlite>>,
) -> Result<Html<String>, (StatusCode, String)> {
    let user = check_auth(
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

    let permissions = vec![
        ("Manage users", "manage_users"),
        ("Upload files", "upload_files"),
        ("List files", "list_files"),
        ("Delete files", "delete_files"),
    ];

    Ok(Html(
        html! {
            (DOCTYPE)
            html {
                (head("New user", None, None))
                body {
                    (nav(Routes::Users, Some(&user.username), Some(user.permissions)))
                    h2 { "New User" }
                    form action="/user" method="post" enctype="multipart/form-data" {
                        label {
                            "Username"
                            input type="text" name="username" required;
                        }
                        label {
                            "Password"
                            input type="password" name="password" required;
                        }
                        label {
                            "Termination"
                            input type="datetime-local" name="terminate";
                        }
                        ul {
                            @for permission in permissions {
                                li {
                                    label {
                                        (permission.0)
                                            input type="checkbox" name=(permission.1);
                                    }
                                }
                            }
                        }
                        input type="submit" value="Create User";
                    }
                }
            }
        }
        .into_string(),
    ))
}
