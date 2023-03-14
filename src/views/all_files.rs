use axum::{extract::State, http::StatusCode, response::Html};
use maud::{html, PreEscaped, DOCTYPE};
use sqlx::{
    types::chrono::{DateTime, NaiveDateTime, Utc},
    Pool, Sqlite,
};

use crate::db::get_files_from_db;

pub async fn all_files(
    State(db): State<Pool<Sqlite>>,
) -> Result<Html<String>, (StatusCode, String)> {
    let files = get_files_from_db(db).await?;

    Ok(Html(
        html! {
            (DOCTYPE)
            head {
                title { "Filehost" }
                link rel="stylesheet" type="text/css" href="assets/styles.css";
            }
            body {
                nav {
                    ul {
                        li { a href="/" { "home" }}
                        li { a href="/upload" { "upload" }}
                        li { a class="current" href="/files" { "files list" }}
                    }
                }
                h2 { "File list" }
                ul class="files-list" {
                    @for file in files {
                        li {
                            div class="header" {
                                form {
                                    button
                                        formenctype="multipart/form-data"
                                        formaction="/delete"
                                        value=(file.saved_name)
                                        formmethod="post"
                                        name="delete"
                                        type="submit"
                                        { img type="image/svg+xml" src="assets/rubbish.svg"; }
                                }
                                h3 { (file.file_name) }
                            }
                            div class="content" {
                                p class="saved-name" { (file.saved_name) }
                            }
                            div class="metadata" {
                                @if file.password_protected {
                                    img type="image/svg+xml" src="assets/key.svg";
                                } @else {
                                    img type="image/svg+xml" src="assets/unlocked.svg";
                                }
                                p { (file.file_type) }
                                @if file.destroy.is_some() {
                                    p class="timestamp" { ( DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp_opt(file.destroy.unwrap(), 0).unwrap(), Utc).format("%Y/%m/%d %H:%M").to_string() ) }
                                }
                            }
                        }
                    }
                }
                script { (PreEscaped(
                    r#"
                    const files = document.querySelectorAll("body > ul > li");

                    for (const file of files) {
                        const button = file.querySelector("button");
                        const span = file.querySelector(".saved-name");

                        button.addEventListener("click", function(e) {
                            e.preventDefault();

                            let formData = new FormData();

                            formData.append("delete", span.innerHTML)
                            
                            fetch("/delete", {
                                method: "POST",
                                body: formData,
                            })
                                .then(function(data) {
                                    if (!data.ok) throw "failed";
                                    button.innerHTML = "✓";
                                    file.classList.add("deleted");
                                })
                                .catch(function() {
                                    button.innerHTML = "𐄂";
                                });
                        });
                    }
                    "#))
                }
            }
        }
        .into_string(),
    ))
}
