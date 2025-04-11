use axum::{extract::State, http::StatusCode, response::Html};
use axum_extra::{TypedHeader, headers::Cookie};
use maud::{DOCTYPE, html};
use sqlx::{Pool, Sqlite};

use crate::{
    db::get_all_files_from_db,
    handlers::{AuthOrBasic, check_auth},
    helpers::link_path,
    models::Permissions,
    views::templates::{Routes, head, nav},
};

// FIXME: file direct link only works if the file isn't password locked
pub async fn all_files(
    TypedHeader(cookie): TypedHeader<Cookie>,
    State(db): State<Pool<Sqlite>>,
) -> Result<Html<String>, (StatusCode, String)> {
    let user = check_auth(
        &db,
        AuthOrBasic::Cookie(cookie),
        Some(Permissions {
            manage_users: false,
            upload_files: false,
            list_files: true,
            delete_files: false,
        }),
    )
    .await?;

    let files = get_all_files_from_db(db).await?;

    Ok(Html(
        html! {
            (DOCTYPE)
            html {
                (head("Files list", Some("assets/list.js"), None))
                body {
                    (nav(Routes::Files, Some(user.username), Some(user.permissions)))
                    h2 { "File list" }
                    ul class="files-list" {
                        @for file in files {
                            li {
                                div class="header" {
                                    form {
                                        button
                                            formenctype="multipart/form-data"
                                            formaction=(link_path("/delete"))
                                            value=(file.saved_name)
                                            formmethod="post"
                                            name="delete"
                                            type="submit"
                                            { img type="image/svg+xml" src="assets/rubbish.svg"; }
                                    }
                                    h3 { (file.file_name) }
                                }
                                div class="content" {
                                    a
                                        href={(file.saved_name.clone())}
                                        class="saved-name"
                                        { (file.saved_name) }
                                }
                                div class="metadata" {
                                    @if file.password.is_some() {
                                        img type="image/svg+xml" src="assets/key.svg";
                                    } @else {
                                        img type="image/svg+xml" src="assets/unlocked.svg";
                                    }
                                    p { (file.file_type) }
                                    @if let Some(timestamp) = file.destroy {
                                        p class="timestamp" {
                                            time datetime=(
                                                timestamp
                                                    .format("%Y-%m-%dT%H:%M:%S")
                                                    .to_string()
                                            ) {
                                                (
                                                    timestamp
                                                        .format("%Y/%m/%d %H:%M")
                                                        .to_string()
                                                )
                                            }
                                        }
                                    }
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
