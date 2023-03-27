use axum::response::Html;
use maud::{html, DOCTYPE};

use crate::views::templates::{head, nav, Routes};

pub async fn sign_in() -> Html<String> {
    Html(
        html! {
            (DOCTYPE)
            html {
                (head("Sign in", None, None))
                body {
                    (nav::<String>(Routes::SignIn, None, None))
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
