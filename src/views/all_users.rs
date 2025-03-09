use axum::{TypedHeader, extract::State, headers::Cookie, http::StatusCode, response::Html};
use maud::{DOCTYPE, PreEscaped, html};
use sqlx::{Pool, Sqlite};

use crate::{
    db::get_users_from_db,
    handlers::{AuthOrBasic, check_auth},
    models::Permissions,
    views::templates::{Routes, head, nav},
};

pub async fn all_users(
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

    let users = get_users_from_db(db).await?;

    Ok(Html(
        html! {
            (DOCTYPE)
            html {
                (head("User list", None, None))
                body {
                    (nav(Routes::Users, Some(user.username), Some(user.permissions)))
                    h2 { "User list" }
                    a href="/user" { "Create new user" }
                    ul class="users-list" {
                        @for user in users {
                            li {
                                div class="header" {
                                    form {
                                        button
                                            formenctype="multipart/form-data"
                                            formaction="/delete-user"
                                            value=(user.username)
                                            formmethod="post"
                                            name="username"
                                            type="submit"
                                            { img type="image/svg+xml" src="assets/rubbish.svg"; }
                                    }
                                    h3 { (user.username) }
                                }
                                form class="content" enctype="multipart/form-data" action="/edit-user" method="post" {
                                    input name="username" value=(user.username) type="hidden";
                                    @if let Some(terminate) = user.terminate {
                                        input name="terminate" value=(terminate.format("%Y-%m-%dT%H:%M:%S").to_string()) type="hidden";
                                    }
                                    label {
                                        "Change password"
                                        input type="password" name="password";
                                    }
                                    ul {
                                        (checkbox("Manage users", "manage_users", user.permissions.manage_users))
                                        (checkbox("Upload files", "upload_files", user.permissions.upload_files))
                                        (checkbox("List files", "list_files", user.permissions.list_files))
                                        (checkbox("Delete files", "delete_files", user.permissions.delete_files))

                                    }
                                    input type="submit" value="Edit user";
                                }
                                div class="metadata" {
                                    @if let Some(timestamp) = user.terminate {
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

fn checkbox(label: &str, name: &str, checked: bool) -> PreEscaped<String> {
    html! {
        li {
            (label)
            @if checked {
                input type="checkbox" name=(name) checked;
            } @else {
                input type="checkbox" name=(name);
            }
        }
    }
}
