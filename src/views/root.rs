use axum::response::Html;
use maud::{html, DOCTYPE};

pub async fn root() -> Html<String> {
    Html(
        html! {
            (DOCTYPE)
            head {
                title { "Filehost" }
            }
            body {
                h1 { "Download File" }
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
