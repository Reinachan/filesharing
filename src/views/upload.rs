use axum::{extract::State, headers::Cookie, http::StatusCode, response::Html, TypedHeader};
use maud::{html, DOCTYPE};
use sqlx::{Pool, Sqlite};

use crate::{
    handlers::{check_auth, AuthOrBasic},
    models::Permissions,
    views::templates::{head, nav, Routes},
};

pub async fn upload(
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

    Ok(Html(
        html! {
            (DOCTYPE)
            html {
                (head("Upload", Some("assets/upload.js"), None))
                body {
                    (nav(Routes::Upload, Some(&user.username), Some(user.permissions)))
                    h2 { "Upload File" }
                    form action="/upload" method="post" enctype="multipart/form-data" {
                        label {
                            "Upload file:"
                            input type="file" name="file";
                        }
                        br;
                        label {
                            "Destroy file at approx:"
                            input type="datetime-local" name="destroy";
                        }
                        br;
                        label {
                            "Add password?"
                            input type="text" name="password";
                        }
                        br;
                        input type="submit" value="Upload file";
                        br;
                        output {}
                    }
                }
            }
        }
        .into_string(),
    ))
}
