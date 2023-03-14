use axum::response::Html;
use maud::{html, DOCTYPE};

pub async fn root() -> Html<String> {
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
                        li { a class="current" href="/" { "home" }}
                        li { a href="/upload" { "upload" }}
                        li { a href="/files" { "files list" }}
                    }
                }
                h2 { "Download File" }
                form action="/" method="post" enctype="multipart/form-data" {
                    label {
                        "Filename"
                        input type="text" name="file_name";
                    }
                    br;
                    label {
                        "Password"
                        input type="text" name="password";
                    }
                    br;
                    input type="submit" value="Download";
                }
            }
        }
        .into_string(),
    )
}
