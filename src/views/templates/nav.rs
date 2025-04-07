use maud::{PreEscaped, Render, html};

use crate::{constants::SERVER_NAME, helpers::link_path, models::Permissions};

#[derive(Debug, PartialEq)]
pub enum Routes {
    Root,
    Upload,
    Files,
    Users,
    Profile,
    SignIn,
}

pub fn nav<S: Render + AsRef<str>>(
    current: Routes,
    username: Option<S>,
    permissions: Option<Permissions>,
) -> PreEscaped<String> {
    html! {
        nav {
            h1 { (*SERVER_NAME) }
            ul {
                (nav_item(&current, &Routes::Root, &link_path("/"), "home"))
                (nav_item(&current, &Routes::Upload, &link_path("/upload"), "upload"))
                (nav_item(&current, &Routes::Files, &link_path("/files"), "files"))
                @if permissions.is_some() && permissions.unwrap().manage_users {
                    (nav_item(&current, &Routes::Users, &link_path("/users"), "users"))
                }
                @match username { Some(username) => {
                    (nav_item(&current, &Routes::Profile, &link_path("/profile"), username))
                } _ => {
                    (nav_item(&current, &Routes::SignIn, &link_path("/signin"), "signin"))
                }}
            }
        }
    }
}

fn nav_item<S: AsRef<str> + Render>(
    current: &Routes,
    route: &Routes,
    href: &str,
    name: S,
) -> PreEscaped<String> {
    html! {
        @if current == route {
            li { a class="current" href=(href) { (name) }}
        } @else {
            li { a href=(href) { (name) }}
        }
    }
}
