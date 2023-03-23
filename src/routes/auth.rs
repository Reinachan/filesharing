use axum::{
    extract::{Multipart, State},
    http::{header::SET_COOKIE, HeaderMap, HeaderValue},
    response::{IntoResponse, Redirect},
};
use http_auth_basic::Credentials;
use sqlx::{Pool, Sqlite};

use crate::{
    constants::AUTH_COOKIE,
    handlers::{check_auth, AuthOrBasic},
};

pub async fn auth(State(db): State<Pool<Sqlite>>, mut multipart: Multipart) -> impl IntoResponse {
    let mut username = String::new();
    let mut password = String::new();

    while let Some(field) = multipart.next_field().await.unwrap() {
        let field_name = field.name().unwrap().to_string();

        if field_name == *"username" {
            username = field.text().await.unwrap();
        } else if field_name == *"password" {
            password = field.text().await.unwrap();
        }
    }

    let _user = match check_auth(
        &db,
        AuthOrBasic::Basic((username.clone(), password.clone())),
        None,
    )
    .await
    {
        Ok(val) => val,
        Err(err) => return Err(err),
    };

    let credentials = Credentials::new(&username, &password);

    let mut headers = HeaderMap::new();
    headers.insert(
        SET_COOKIE,
        HeaderValue::from_str(format!("{}={}", AUTH_COOKIE, credentials.as_http_header()).as_str())
            .unwrap(),
    );

    Ok((headers, Redirect::to("/upload")))
}
