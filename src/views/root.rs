use axum::response::Html;
use maud::{html, DOCTYPE};

use crate::views::templates::{head, nav, Routes};

pub async fn root() -> Html<String> {
    Html(
        html! {
            (DOCTYPE)
            html {
                (head("Download", None, None))
                body {
                    (nav::<String>(Routes::Root, None, None))
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
        }
        .into_string(),
    )
}
