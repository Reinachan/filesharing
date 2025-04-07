use maud::{PreEscaped, html};

use crate::{constants::SERVER_NAME, helpers::link_path};

pub fn head<S: AsRef<str> + std::fmt::Display>(
    page: S,
    script: Option<&str>,
    additional: Option<PreEscaped<String>>,
) -> PreEscaped<String> {
    html! {
        head {
            title { (format!("{} - {}", *SERVER_NAME, page)) }
            link rel="stylesheet" type="text/css" href=(link_path("/assets/styles.css"));
            meta name="viewport" content="width=device-width, initial-scale=1.0";
            link rel="shortcut icon" href="assets/favicon.svg" type="image/svg";
            @if let Some(script) = script {
                script src=(script) defer {}
            }
            @if let Some(additional) = additional {
                (additional)
            }
        }
    }
}
