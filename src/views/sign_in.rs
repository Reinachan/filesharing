use axum::response::Html;
use maud::{html, DOCTYPE};

pub async fn sign_in() -> Html<String> {
    Html(
        html! {
            (DOCTYPE)
            html {
                head {
                    title { "Filehost" }
                    link rel="stylesheet" type="text/css" href="assets/styles.css";
                    meta name="viewport" content="width=device-width, initial-scale=1.0";
                }
                body {
                    nav {
                        ul {
                            li { a href="/" { "home" }}
                            li { a href="/upload" { "upload" }}
                            li { a href="/files" { "files list" }}
                            li { a class="current" href="/signin" { "sign in" }}
                        }
                    }
                    h2 { "Sign In" }
                    form action="/auth" method="post" enctype="multipart/form-data" {
                        label {
                            "Username"
                            input type="text" name="username";
                        }
                        br;
                        label {
                            "Password"
                            input type="password" name="password";
                        }
                        br;
                        input type="submit" value="sign in";
                    }
                }
            }
        }
        .into_string(),
    )
}
