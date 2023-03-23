use axum::{extract::State, headers::Cookie, response::Html, TypedHeader};
use maud::{html, DOCTYPE};
use sqlx::{Pool, Sqlite};

use crate::{
    handlers::{check_auth, AuthOrBasic},
    models::Permissions,
};

pub async fn upload(
    TypedHeader(cookie): TypedHeader<Cookie>,
    State(db): State<Pool<Sqlite>>,
) -> Html<String> {
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
    .await;

    Html(
        html! {
            (DOCTYPE)
            html {
                head {
                    title { "Filehost" }
                    link rel="stylesheet" type="text/css" href="assets/styles.css";
                    script src="assets/upload.js" defer {}
                    meta name="viewport" content="width=device-width, initial-scale=1.0";
                }
                body {
                    nav {
                        ul {
                            li { a href="/" { "home" }}
                            li { a class="current" href="/upload" { "upload" }}
                            li { a href="/files" { "files list" }}
                            @if user.is_ok() {
                                li { a href="/profile" { (user.unwrap().username) }}
                            } @else {
                                li { a href="/signin" { "sign in" }}
                            }
                        }
                    }
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
    )
}
