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

use crate::models::{Permissions, PermissionsDB, User, UserDB};

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
    let user = retrieve_user_by_username(&user_data.username, &db).await?;

    if !verify(&user_data.password, &user.password)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    {
        return Err(StatusCode::UNAUTHORIZED);
    }

    let token = encode_jwt(user.username).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(TokenResponse { token }))
}

/// Retrives the user data from the database. Includes the permissions and the password hash
async fn retrieve_user_by_username(username: &str, db: &Pool<Sqlite>) -> Result<User, StatusCode> {
    let user = sqlx::query_as!(
        UserDB,
        "
        SELECT * FROM users WHERE username = ?
        ",
        username
    )
    .fetch_one(db)
    .await
    .map_err(|_| (StatusCode::UNAUTHORIZED))?;

    let permissions = sqlx::query_as!(
        PermissionsDB,
        "
        SELECT * FROM permissions WHERE username = ?
        ",
        username
    )
    .fetch_one(db)
    .await
    .unwrap_or(PermissionsDB {
        username: user.username.clone(),
        manage_users: false,
        upload_files: false,
        list_files: false,
        delete_files: false,
    });

    let permissions: Permissions = Permissions {
        manage_users: permissions.manage_users,
        upload_files: permissions.upload_files,
        list_files: permissions.list_files,
        delete_files: permissions.delete_files,
    };

    Ok(User {
        username: user.username,
        password: user.password,
        terminate: user.terminate,
        permissions,
    }) // Return the hardcoded user
}

pub fn encode_jwt(username: String) -> Result<String, StatusCode> {
    let secret = env!("JWT_TOKEN_SECRET");

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
    let secret = env!("JWT_TOKEN_SECRET");
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

    let token_data = match decode_jwt(token.unwrap().to_string()) {
        Ok(data) => data,
        Err(_) => {
            return Err((
                StatusCode::UNAUTHORIZED,
                "Unable to decode token".to_string(),
            ));
        }
    };

    let user = retrieve_user_by_username(&token_data.claims.username, &db)
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
