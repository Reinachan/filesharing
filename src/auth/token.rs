use axum::{
    body::Body,
    extract::{Json, Request, State},
    http::{self, Response, StatusCode},
    middleware::Next,
};
use bcrypt::verify;
use chrono::{Duration, Utc};
// use futures::TryFutureExt;
use jsonwebtoken::{DecodingKey, EncodingKey, Header, TokenData, Validation, decode, encode};
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Sqlite};

use crate::db::get_user_by_username;

// Define a structure for holding claims data used in JWT tokens
#[derive(Serialize, Deserialize)]
pub struct Claims {
    pub exp: usize,       // Expiry time of the token
    pub iat: usize,       // Issued at time of the token
    pub username: String, // Username associated with the token
}

#[derive(Deserialize)]
pub struct SignInData {
    pub username: String,
    pub password: String,
}

#[derive(serde::Serialize)]
pub struct TokenResponse {
    token: String,
}

/// Function to handle sign-in requests
pub async fn request_token(
    State(db): State<Pool<Sqlite>>,
    Json(user_data): Json<SignInData>,
) -> Result<Json<TokenResponse>, StatusCode> {
    let user = get_user_by_username(&user_data.username, &db)
        .await
        .map_err(|_| StatusCode::UNAUTHORIZED)?;

    if !verify(&user_data.password, &user.password)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    {
        return Err(StatusCode::UNAUTHORIZED);
    }

    let token = encode_jwt(user.username).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(TokenResponse { token }))
}

pub fn encode_jwt(username: String) -> Result<String, StatusCode> {
    let secret = std::env::var("JWT_TOKEN_SECRET")
        .expect("You need to configure a JWT_TOKEN_SECRET env var");

    let now = Utc::now();

    // TODO: Implement refresh token so we can reduce the duration
    let expire: chrono::TimeDelta = Duration::weeks(4);

    let exp: usize = (now + expire).timestamp() as usize;
    let iat: usize = now.timestamp() as usize;

    let claim = Claims { iat, exp, username };

    encode(
        &Header::default(),
        &claim,
        &EncodingKey::from_secret(secret.as_ref()),
    )
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

pub fn decode_jwt(jwt_token: String) -> Result<TokenData<Claims>, StatusCode> {
    let secret = std::env::var("JWT_TOKEN_SECRET")
        .expect("You need to configure a JWT_TOKEN_SECRET env var");

    let result: Result<TokenData<Claims>, StatusCode> = decode(
        &jwt_token,
        &DecodingKey::from_secret(secret.as_ref()),
        &Validation::default(),
    )
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR);
    result
}

/// Authorization middleware to ensure the user is authorised to fetch the page
///
/// # Examples
///
/// The basics of using it goes like this
///
/// ```
///     let app = Router::new()
///         .route(
///             "/api/users",
///                 get(api_get_users).layer(middleware::from_fn_with_state(
///                     conn.clone(),
///                     authorization_middleware,
///             )),
///         )
/// ```
///
/// You can then access the user's information and permissions within the route like this using the Extension<User> extractor
/// ```
/// pub async fn some_route(Extension(user): Extension<User>) -> impl IntoResponse {
///     Json(user)
/// }
/// ```
pub async fn authorization_middleware(
    State(db): State<Pool<Sqlite>>,
    mut req: Request,
    next: Next,
) -> Result<Response<Body>, (StatusCode, String)> {
    let auth_header = req.headers_mut().get(http::header::AUTHORIZATION);

    let auth_header = match auth_header {
        Some(header) => header.to_str().map_err(|_| {
            (
                StatusCode::FORBIDDEN,
                "Empty header is not allowed".to_string(),
            )
        })?,
        None => {
            return Err((
                StatusCode::FORBIDDEN,
                "Please add the JWT token to the header".to_string(),
            ));
        }
    };

    let mut header = auth_header.split_whitespace();

    let (_bearer, token) = (header.next(), header.next());

    let token = match token {
        Some(token) => token,
        None => {
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                "Token cannot be parsed".to_string(),
            ));
        }
    };

    let token_data = match decode_jwt(token.to_string()) {
        Ok(data) => data,
        Err(_) => {
            return Err((
                StatusCode::UNAUTHORIZED,
                "Unable to decode token".to_string(),
            ));
        }
    };

    let user = get_user_by_username(&token_data.claims.username, &db)
        .await
        .map_err(|_| {
            (
                StatusCode::FORBIDDEN,
                "You are not an authorized user".to_string(),
            )
        })?;

    req.extensions_mut().insert(user);
    Ok(next.run(req).await)
}
