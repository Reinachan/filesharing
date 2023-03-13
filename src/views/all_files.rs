use axum::{extract::State, http::StatusCode, response::Html};
use maud::{html, PreEscaped, DOCTYPE};
use sqlx::{Pool, Sqlite};

use crate::actions::get_files_from_db;

pub async fn all_files(
    State(db): State<Pool<Sqlite>>,
) -> Result<Html<String>, (StatusCode, String)> {
    let files = get_files_from_db(db).await?;

    Ok(Html(
        html! {
            (DOCTYPE)
            head {
                title { "Filehost" }
            }
            body {
                h1 { "File list" }
                ul {
                    @for file in files {
                        li {
                            span { (file.file_name) }
                            button { "delete" }
                        }
                    }
                }
                script { (PreEscaped(
                    r#"
                    const files = document.querySelectorAll("li");

                    for (const file of files) {
                        const button = file.querySelector("button");
                        const span = file.querySelector("span");

                        button.addEventListener("click", function() {
                            fetch(`/delete/${encodeURI(span.innerHTML)}`, {
                                method: "DELETE"
                            })
                                .then(function(data) {
                                    if (!data.ok) throw "failed";
                                    button.innerHTML = "deleted";
                                })
                                .catch(function() {
                                    button.innerHTML = "failed";
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
