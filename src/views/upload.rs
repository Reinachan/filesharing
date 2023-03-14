use axum::response::Html;
use maud::{html, DOCTYPE};

pub async fn upload() -> Html<String> {
    Html(
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
                        li { a class="current" href="/upload" { "upload" }}
                        li { a href="/files" { "files list" }}
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
                }
            }
        }
        .into_string(),
    )
}
